use crate::router_setup::{json_response, upper_bound, CrmState};
use akurai_http::{Request, Response};
use akurai_json::Value;
use std::sync::{Arc, Mutex};

pub fn timeline_route(state: Arc<Mutex<CrmState>>) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let query = req.query.as_deref().unwrap_or("");
        let mut entity_type = String::new();
        let mut entity_id: u64 = 0;

        for pair in query.split('&') {
            let mut parts = pair.splitn(2, '=');
            match parts.next() {
                Some("entityType") => entity_type = parts.next().unwrap_or("").to_string(),
                Some("entityId") => entity_id = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0),
                _ => {}
            }
        }

        if entity_type.is_empty() || entity_id == 0 {
            return json_response(Value::Object(vec![
                ("timeline".into(), Value::Array(vec![])),
            ]));
        }

        let state = state.lock().unwrap();
        let mut db = state.db.lock().unwrap();
        let mut activities = Vec::new();

        let prefix = format!("timeline:{}:{}:", entity_type, entity_id).into_bytes();
        let end = upper_bound(&prefix);
        if let Ok(entries) = db.range(&prefix, &end) {
            for (_key_bytes, val_bytes) in entries {
                if let Ok(json) = akurai_json::parse(std::str::from_utf8(&val_bytes).unwrap_or("")) {
                    activities.push(json);
                }
            }
        }

        activities.reverse();

        json_response(Value::Object(vec![
            ("entityType".into(), Value::Str(entity_type)),
            ("entityId".into(), Value::Int(entity_id as i64)),
            ("timeline".into(), Value::Array(activities)),
        ]))
    })
}

pub fn record_timeline(
    db: &mut akurai_storage::BTree,
    entity_type: &str,
    entity_id: u64,
    action: &str,
    actor_id: Option<u64>,
) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;

    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let rand_suffix = RandomState::new().build_hasher().finish() % 10000;
    let key = format!("timeline:{}:{}:{}_{}", entity_type, entity_id, now, rand_suffix).into_bytes();

    let record = Value::Object(vec![
        ("action".into(), Value::Str(action.into())),
        ("entityType".into(), Value::Str(entity_type.into())),
        ("entityId".into(), Value::Int(entity_id as i64)),
        ("actorId".into(), Value::Int(actor_id.unwrap_or(0) as i64)),
        ("createdAt".into(), Value::Int(now)),
    ]);

    let json_str = record.to_json();
    db.insert(&key, json_str.as_bytes())
        .map_err(|e| format!("timeline write: {e}"))?;

    Ok(())
}
