use crate::handlers::timeline::record_timeline;
use crate::router_setup::{bad_request, internal_error, json_response, not_found, upper_bound, CrmState};
use akurai_http::{Request, Response};
use akurai_json::Value;
use std::sync::{Arc, Mutex};

fn parse_body(req: &Request) -> Result<Value, String> {
    if req.body.is_empty() {
        return Err("empty request body".into());
    }
    let text = std::str::from_utf8(&req.body).map_err(|e| format!("body not UTF-8: {e}"))?;
    akurai_json::parse(text).map_err(|e| format!("invalid JSON: {e}"))
}

fn extract_id(path: &str) -> Option<u64> {
    let segments: Vec<&str> = path.split('/').collect();
    segments.last().and_then(|s| s.parse::<u64>().ok())
}

fn collection_name(entity: &str) -> &str {
    match entity {
        "people" => "people",
        "companies" => "companies",
        "opportunities" => "opportunities",
        "tasks" => "tasks",
        "notes" => "notes",
        _ => entity,
    }
}

fn build_key(coll_name: &str, id: u64) -> Vec<u8> {
    let mut key = format!("{}:", coll_name).into_bytes();
    key.extend_from_slice(&id.to_be_bytes());
    key
}

pub fn list_route(state: Arc<Mutex<CrmState>>, entity: &'static str) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |_req: &Request| {
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return internal_error("lock poisoned"),
        };
        let coll_name = collection_name(entity);
        let mut db = match state.db.lock() {
            Ok(db) => db,
            Err(_) => return internal_error("db lock poisoned"),
        };

        let prefix = format!("{}:", coll_name).into_bytes();
        let end = upper_bound(&prefix);
        let mut records = Vec::new();

        if let Ok(entries) = db.range(&prefix, &end) {
            for (_key_bytes, val_bytes) in entries {
                if let Ok(json) = akurai_json::parse(std::str::from_utf8(&val_bytes).unwrap_or("")) {
                    records.push(json);
                }
            }
        }

        json_response(Value::Array(records))
    })
}

pub fn get_route(state: Arc<Mutex<CrmState>>, entity: &'static str) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let id = match extract_id(&req.path) {
            Some(id) => id,
            None => return bad_request("missing id"),
        };
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return internal_error("lock poisoned"),
        };
        let coll_name = collection_name(entity);
        let mut db = match state.db.lock() {
            Ok(db) => db,
            Err(_) => return internal_error("db lock poisoned"),
        };

        let key = build_key(coll_name, id);
        match db.get(&key) {
            Ok(Some(val)) => {
                match akurai_json::parse(std::str::from_utf8(&val).unwrap_or("")) {
                    Ok(json) => json_response(json),
                    Err(_) => internal_error("corrupted data"),
                }
            }
            Ok(None) => not_found(&format!("{entity}/{id}")),
            Err(e) => internal_error(&format!("storage: {e}")),
        }
    })
}

pub fn create_route(state: Arc<Mutex<CrmState>>, entity: &'static str) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let body = match parse_body(req) {
            Ok(v) => v,
            Err(e) => return bad_request(&e),
        };
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return internal_error("lock poisoned"),
        };
        let coll_name = collection_name(entity);
        let mut db = match state.db.lock() {
            Ok(db) => db,
            Err(_) => return internal_error("db lock poisoned"),
        };

        let now = CrmState::now();
        let counter_key = format!("{}:_counter", coll_name).into_bytes();
        let next_id = match db.get(&counter_key) {
            Ok(Some(bytes)) => {
                let s = String::from_utf8_lossy(&bytes).to_string();
                match s.parse::<u64>() {
                    Ok(id) if id < u64::MAX => id + 1,
                    _ => return internal_error("ID space exhausted"),
                }
            }
            _ => 1,
        };
        if let Err(e) = db.insert(&counter_key, next_id.to_string().as_bytes()) {
            return internal_error(&format!("counter: {e}"));
        }
        if let Err(e) = db.commit() {
            return internal_error(&format!("commit: {e}"));
        }

        let mut obj = match body {
            Value::Object(fields) => fields,
            _ => return bad_request("expected JSON object"),
        };
        obj.push(("id".into(), Value::Int(next_id as i64)));
        obj.push(("createdAt".into(), Value::Int(now)));
        obj.push(("updatedAt".into(), Value::Int(now)));

        let record = Value::Object(obj);
        let key = build_key(coll_name, next_id);
        let json_str = record.to_json();

        match db.insert(&key, json_str.as_bytes()) {
            Ok(_) => {
                if let Err(e) = db.commit() {
                    return internal_error(&format!("commit: {e}"));
                }
                let _ = record_timeline(&mut db, coll_name, next_id, "created", None);
                let body = record.to_json();
                Response::new(201)
                    .with_header("Content-Type", "application/json")
                    .with_header("Access-Control-Allow-Origin", "*")
                    .with_body("application/json", body.into_bytes())
            }
            Err(e) => internal_error(&format!("storage: {e}")),
        }
    })
}

pub fn update_route(state: Arc<Mutex<CrmState>>, entity: &'static str) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let id = match extract_id(&req.path) {
            Some(id) => id,
            None => return bad_request("missing id"),
        };
        let body = match parse_body(req) {
            Ok(v) => v,
            Err(e) => return bad_request(&e),
        };
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return internal_error("lock poisoned"),
        };
        let coll_name = collection_name(entity);
        let mut db = match state.db.lock() {
            Ok(db) => db,
            Err(_) => return internal_error("db lock poisoned"),
        };

        let key = build_key(coll_name, id);

        let existing = match db.get(&key) {
            Ok(Some(val)) => {
                match akurai_json::parse(std::str::from_utf8(&val).unwrap_or("")) {
                    Ok(v) => v,
                    Err(_) => return internal_error("corrupted data"),
                }
            }
            Ok(None) => return not_found(&format!("{entity}/{id}")),
            Err(e) => return internal_error(&format!("storage: {e}")),
        };

        let mut existing_fields = match existing {
            Value::Object(f) => f,
            _ => return internal_error("corrupted record"),
        };

        if let Value::Object(update_fields) = body {
            for (k, v) in update_fields {
                if k != "id" && k != "createdAt" {
                    let mut found = false;
                    for pair in existing_fields.iter_mut() {
                        if pair.0 == k {
                            pair.1 = v.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        existing_fields.push((k.clone(), v.clone()));
                    }
                }
            }
        }

        let now = CrmState::now();
        let mut found_updated = false;
        for pair in existing_fields.iter_mut() {
            if pair.0 == "updatedAt" {
                pair.1 = Value::Int(now);
                found_updated = true;
                break;
            }
        }
        if !found_updated {
            existing_fields.push(("updatedAt".into(), Value::Int(now)));
        }

        let record = Value::Object(existing_fields);
        let json_str = record.to_json();

        match db.insert(&key, json_str.as_bytes()) {
            Ok(_) => {
                if let Err(e) = db.commit() {
                    return internal_error(&format!("commit: {e}"));
                }
                let _ = record_timeline(&mut db, coll_name, id, "updated", None);
                json_response(record)
            }
            Err(e) => internal_error(&format!("storage: {e}")),
        }
    })
}

pub fn delete_route(state: Arc<Mutex<CrmState>>, entity: &'static str) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let id = match extract_id(&req.path) {
            Some(id) => id,
            None => return bad_request("missing id"),
        };
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return internal_error("lock poisoned"),
        };
        let coll_name = collection_name(entity);
        let mut db = match state.db.lock() {
            Ok(db) => db,
            Err(_) => return internal_error("db lock poisoned"),
        };

        let key = build_key(coll_name, id);
        match db.delete(&key) {
            Ok(true) => {
                if let Err(e) = db.commit() {
                    return internal_error(&format!("commit: {e}"));
                }
                let _ = record_timeline(&mut db, coll_name, id, "deleted", None);
                json_response(Value::Object(vec![
                    ("deleted".into(), Value::Bool(true)),
                    ("id".into(), Value::Int(id as i64)),
                ]))
            }
            Ok(false) => not_found(&format!("{entity}/{id}")),
            Err(e) => internal_error(&format!("storage: {e}")),
        }
    })
}
