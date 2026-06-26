use akurai_json::Value;
use akurai_storage::BTree;

pub struct User {
    pub sub: String,
    pub email: String,
}

pub fn create(db: &mut BTree, sub: &str, email: &str) -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let rand = RandomState::new().build_hasher().finish();
    let session_id = format!("s_{:x}_{}", rand, now);

    let record = Value::Object(vec![
        ("sub".into(), Value::Str(sub.into())),
        ("email".into(), Value::Str(email.into())),
        ("createdAt".into(), Value::Int(now as i64)),
    ]);

    let key = format!("_session:{}", session_id);
    let _ = db.insert(key.as_bytes(), record.to_json().as_bytes());
    let _ = db.commit();

    session_id
}

pub fn extract_session_id(req: &akurai_http::Request) -> Option<String> {
    if let Some(auth) = req.header("Authorization") {
        if let Some(token) = auth.strip_prefix("Bearer ") {
            let t = token.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }

    if let Some(cookies) = req.header("Cookie") {
        for part in cookies.split(';') {
            let part = part.trim();
            if let Some(value) = part.strip_prefix("crm_session=") {
                let v = value.trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }
    }

    None
}

pub fn get_user(db: &mut BTree, session_id: &str) -> Option<User> {
    let key = format!("_session:{}", session_id);
    match db.get(key.as_bytes()) {
        Ok(Some(bytes)) => {
            let s = String::from_utf8_lossy(&bytes);
            if let Ok(val) = akurai_json::parse(&s) {
                let sub = val.get("sub").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let email = val.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
                Some(User { sub, email })
            } else {
                None
            }
        }
        _ => None,
    }
}
