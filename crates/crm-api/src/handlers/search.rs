use crate::router_setup::{internal_error, json_response, upper_bound, CrmState};
use akurai_http::{Request, Response};
use akurai_json::Value;
use std::sync::{Arc, Mutex};

fn url_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hi = chars.next().and_then(|c| c.to_digit(16)).unwrap_or(0);
            let lo = chars.next().and_then(|c| c.to_digit(16)).unwrap_or(0);
            result.push(char::from((hi * 16 + lo) as u8));
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    result
}

fn json_values_contain(val: &akurai_json::Value, term: &str) -> bool {
    match val {
        akurai_json::Value::Str(s) => s.to_lowercase().contains(term),
        akurai_json::Value::Object(pairs) => pairs.iter().any(|(_, v)| json_values_contain(v, term)),
        akurai_json::Value::Array(items) => items.iter().any(|v| json_values_contain(v, term)),
        _ => false,
    }
}

pub fn search_route(state: Arc<Mutex<CrmState>>) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let query = req.query.as_deref().unwrap_or("");
        let search_term = query.split('&')
            .find_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                if parts.next()? == "q" {
                    parts.next().map(|v| url_decode(v).to_lowercase())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        if search_term.is_empty() {
            return json_response(Value::Object(vec![
                ("query".into(), Value::Str("".into())),
                ("results".into(), Value::Array(vec![])),
            ]));
        }

        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return internal_error("lock poisoned"),
        };
        let mut db = match state.db.lock() {
            Ok(db) => db,
            Err(_) => return internal_error("db lock poisoned"),
        };
        let mut results = Vec::new();

        let entity_prefixes = ["people:", "companies:", "opportunities:", "tasks:", "notes:"];
        for prefix_str in &entity_prefixes {
            let prefix = prefix_str.as_bytes();
            let end = upper_bound(prefix);
            if let Ok(entries) = db.range(prefix, &end) {
                for (_key_bytes, val_bytes) in entries {
                    if let Ok(json) = akurai_json::parse(std::str::from_utf8(&val_bytes).unwrap_or("")) {
                        if json_values_contain(&json, &search_term) {
                            let entry = vec![
                                ("entity".into(), Value::Str(prefix_str.trim_end_matches(':').into())),
                                ("record".into(), json),
                            ];
                            results.push(Value::Object(entry));
                        }
                    }
                }
            }
        }

        json_response(Value::Object(vec![
            ("query".into(), Value::Str(search_term)),
            ("results".into(), Value::Array(results)),
        ]))
    })
}
