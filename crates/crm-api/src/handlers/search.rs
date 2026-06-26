use crate::router_setup::{json_response, upper_bound, CrmState};
use akurai_http::{Request, Response};
use akurai_json::Value;
use std::sync::{Arc, Mutex};

pub fn search_route(state: Arc<Mutex<CrmState>>) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let query = req.query.as_deref().unwrap_or("");
        let search_term = query.split('&')
            .find_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                if parts.next()? == "q" {
                    parts.next().map(|v| v.to_lowercase())
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

        let state = state.lock().unwrap();
        let mut db = state.db.lock().unwrap();
        let mut results = Vec::new();

        let entity_prefixes = ["people:", "companies:", "opportunities:", "tasks:", "notes:"];
        for prefix_str in &entity_prefixes {
            let prefix = prefix_str.as_bytes();
            let end = upper_bound(prefix);
            if let Ok(entries) = db.range(prefix, &end) {
                for (_key_bytes, val_bytes) in entries {
                    let val = String::from_utf8_lossy(&val_bytes).to_string();
                    if let Ok(json) = akurai_json::parse(std::str::from_utf8(&val_bytes).unwrap_or("")) {
                        let val_lower = val.to_lowercase();
                        if val_lower.contains(&search_term) {
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
