use akurai_json::Value;
use crm_api::CrmState;

pub fn run(db_path: &str) -> Result<(), String> {
    println!("🌱 Seeding CRM database: {db_path}");

    let frontend_dir = std::path::PathBuf::from("site/frontend");
    let state = CrmState::new(db_path, frontend_dir)?;
    let mut db = state.db.lock().map_err(|e| format!("lock: {e}"))?;

    let mut insert = |entity: &str, id: u64, fields: Vec<(&str, Value)>| -> Result<(), String> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        let mut obj = Vec::new();
        obj.push(("id".into(), Value::Int(id as i64)));
        obj.push(("createdAt".into(), Value::Int(now)));
        obj.push(("updatedAt".into(), Value::Int(now)));
        for (k, v) in fields {
            obj.push((k.to_string(), v));
        }
        let record = Value::Object(obj);
        let key = format!("{}:{}", entity, id);
        let json = record.to_json();
        db.insert(key.as_bytes(), json.as_bytes())
            .map_err(|e| format!("insert {key}: {e}"))
    };

    // People
    insert(
        "people",
        1,
        vec![
            ("firstName", Value::Str("Anna".into())),
            ("lastName", Value::Str("Jónsdóttir".into())),
            ("email", Value::Str("anna@example.is".into())),
            ("jobTitle", Value::Str("CEO".into())),
            ("companyId", Value::Int(1)),
        ],
    )?;
    insert(
        "people",
        2,
        vec![
            ("firstName", Value::Str("Björn".into())),
            ("lastName", Value::Str("Guðmundsson".into())),
            ("email", Value::Str("bjorn@example.is".into())),
            ("jobTitle", Value::Str("CTO".into())),
            ("companyId", Value::Int(1)),
        ],
    )?;
    insert(
        "people",
        3,
        vec![
            ("firstName", Value::Str("Dóra".into())),
            ("lastName", Value::Str("Sigurðardóttir".into())),
            ("email", Value::Str("dora@acme.is".into())),
            ("jobTitle", Value::Str("VP Engineering".into())),
            ("companyId", Value::Int(2)),
        ],
    )?;
    insert(
        "people",
        4,
        vec![
            ("firstName", Value::Str("Einar".into())),
            ("lastName", Value::Str("Pálsson".into())),
            ("email", Value::Str("einar@startup.is".into())),
            ("jobTitle", Value::Str("Founder".into())),
        ],
    )?;
    insert(
        "people",
        5,
        vec![
            ("firstName", Value::Str("Fríða".into())),
            ("lastName", Value::Str("Jónsdóttir".into())),
            ("email", Value::Str("frida@nordic.is".into())),
            ("jobTitle", Value::Str("Sales Director".into())),
            ("companyId", Value::Int(3)),
        ],
    )?;

    println!("  ✓ 5 people created");

    // Companies
    insert(
        "companies",
        1,
        vec![
            ("name", Value::Str("AkurAI".into())),
            ("domainName", Value::Str("akurai.is".into())),
            ("annualRevenue", Value::Int(50_000_000)),
            ("employeeCount", Value::Int(25)),
            ("websiteUrl", Value::Str("https://akurai.is".into())),
        ],
    )?;
    insert(
        "companies",
        2,
        vec![
            ("name", Value::Str("Acme Corp".into())),
            ("domainName", Value::Str("acme.is".into())),
            ("annualRevenue", Value::Int(1_200_000_000)),
            ("employeeCount", Value::Int(150)),
        ],
    )?;
    insert(
        "companies",
        3,
        vec![
            ("name", Value::Str("Nordic Technologies".into())),
            ("domainName", Value::Str("nordic.is".into())),
            ("annualRevenue", Value::Int(850_000_000)),
            ("employeeCount", Value::Int(75)),
        ],
    )?;
    insert(
        "companies",
        4,
        vec![
            ("name", Value::Str("Startup Iceland".into())),
            ("domainName", Value::Str("startup.is".into())),
            ("annualRevenue", Value::Int(50_000_000)),
            ("employeeCount", Value::Int(5)),
        ],
    )?;

    println!("  ✓ 4 companies created");

    // Opportunities
    insert(
        "opportunities",
        1,
        vec![
            ("name", Value::Str("AkurAI Platform Deal".into())),
            ("amount", Value::Int(25_000_000)),
            ("stage", Value::Str("negotiation".into())),
            ("probability", Value::Int(70)),
            ("personId", Value::Int(1)),
            ("companyId", Value::Int(1)),
        ],
    )?;
    insert(
        "opportunities",
        2,
        vec![
            ("name", Value::Str("Acme Enterprise License".into())),
            ("amount", Value::Int(50_000_000)),
            ("stage", Value::Str("proposal".into())),
            ("probability", Value::Int(50)),
            ("personId", Value::Int(2)),
            ("companyId", Value::Int(2)),
        ],
    )?;
    insert(
        "opportunities",
        3,
        vec![
            ("name", Value::Str("Nordic Consulting".into())),
            ("amount", Value::Int(8_500_000)),
            ("stage", Value::Str("meeting".into())),
            ("probability", Value::Int(30)),
            ("personId", Value::Int(5)),
            ("companyId", Value::Int(3)),
        ],
    )?;
    insert(
        "opportunities",
        4,
        vec![
            ("name", Value::Str("Startup Iceland Partnership".into())),
            ("amount", Value::Int(2_500_000)),
            ("stage", Value::Str("new".into())),
            ("probability", Value::Int(10)),
            ("personId", Value::Int(4)),
            ("companyId", Value::Int(4)),
        ],
    )?;
    insert(
        "opportunities",
        5,
        vec![
            ("name", Value::Str("Lost Deal - Legacy Migrate".into())),
            ("amount", Value::Int(10_000_000)),
            ("stage", Value::Str("lost".into())),
            ("probability", Value::Int(0)),
        ],
    )?;

    println!("  ✓ 5 opportunities created");

    // Tasks
    insert(
        "tasks",
        1,
        vec![
            (
                "title",
                Value::Str("Follow up with Anna about AkurAI deal".into()),
            ),
            ("status", Value::Str("in_progress".into())),
            (
                "dueAt",
                Value::Int(
                    (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as i64)
                        + 86400 * 3,
                ),
            ),
        ],
    )?;
    insert(
        "tasks",
        2,
        vec![
            ("title", Value::Str("Prepare proposal for Acme Corp".into())),
            ("status", Value::Str("todo".into())),
            (
                "dueAt",
                Value::Int(
                    (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as i64)
                        + 86400 * 7,
                ),
            ),
        ],
    )?;
    insert(
        "tasks",
        3,
        vec![
            (
                "title",
                Value::Str("Send NDA to Nordic Technologies".into()),
            ),
            ("status", Value::Str("done".into())),
        ],
    )?;
    insert(
        "tasks",
        4,
        vec![
            ("title", Value::Str("Review Q3 pipeline forecast".into())),
            ("status", Value::Str("todo".into())),
            (
                "dueAt",
                Value::Int(
                    (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as i64)
                        + 86400 * 14,
                ),
            ),
        ],
    )?;

    println!("  ✓ 4 tasks created");

    // Notes
    insert("notes", 1, vec![
        ("title", Value::Str("Initial meeting with Anna".into())),
        ("body", Value::Str("Anna is very interested in our platform. Key concerns are security and compliance. Needs board approval for deals over $200k. Follow up next week with technical deep-dive for Björn.".into())),
    ])?;
    insert("notes", 2, vec![
        ("title", Value::Str("Acme Corp requirements".into())),
        ("body", Value::Str("Acme needs: SSO/SAML integration, custom fields for their industry, API rate limits at 10k/min minimum. Decision maker is Björn, budget holder is CFO Kristín.".into())),
    ])?;

    println!("  ✓ 2 notes created");
    println!("✅ Database seeded successfully!");

    Ok(())
}
