use akurai_json::Value;
use crm_api::CrmState;
use std::path::PathBuf;

#[test]
fn test_create_and_list_people() {
    let db_path = "_test_people.db";
    let _ = std::fs::remove_file(db_path);

    let state = CrmState::new(db_path, PathBuf::from("site/frontend")).unwrap();
    let mut db = state.db.lock().unwrap();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let record = Value::Object(vec![
        ("id".into(), Value::Int(1)),
        ("firstName".into(), Value::Str("Test".into())),
        ("lastName".into(), Value::Str("User".into())),
        ("email".into(), Value::Str("test@test.is".into())),
        ("createdAt".into(), Value::Int(now)),
        ("updatedAt".into(), Value::Int(now)),
    ]);

    db.insert(b"people:1", record.to_json().as_bytes()).unwrap();

    let got = db.get(b"people:1").unwrap().unwrap();
    let got_str = String::from_utf8_lossy(&got);
    assert!(got_str.contains("Test"));
    assert!(got_str.contains("test@test.is"));

    let mut found = false;
    if let Ok(entries) = db.range(b"people:", b"people:\xff") {
        for (key, _) in entries {
            if key == b"people:1" {
                found = true;
            }
        }
    }
    assert!(found, "should find the person in range scan");

    db.delete(b"people:1").unwrap();
    assert!(db.get(b"people:1").unwrap().is_none());

    let _ = std::fs::remove_file(db_path);
}
