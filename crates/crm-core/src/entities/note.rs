use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub id: RecordId,
    pub title: Option<String>,
    pub body: String, // rich text / markdown
    pub person_ids: Vec<RecordId>,
    pub company_ids: Vec<RecordId>,
    pub opportunity_ids: Vec<RecordId>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub created_by: Option<RecordId>,
}

impl Note {
    pub fn to_json(&self) -> akurai_json::Value {
        use akurai_json::Value;
        let mut fields = Vec::new();
        fields.push(("id".into(), Value::Int(self.id as i64)));
        if let Some(ref t) = self.title {
            fields.push(("title".into(), Value::Str(t.clone())));
        }
        fields.push(("body".into(), Value::Str(self.body.clone())));
        actions_json_field(&mut fields, "personIds", &self.person_ids);
        actions_json_field(&mut fields, "companyIds", &self.company_ids);
        actions_json_field(&mut fields, "opportunityIds", &self.opportunity_ids);
        fields.push(("createdAt".into(), Value::Int(self.created_at)));
        fields.push(("updatedAt".into(), Value::Int(self.updated_at)));
        Value::Object(fields)
    }

    pub fn from_json(id: RecordId, val: &akurai_json::Value) -> Result<Self, crate::CoreError> {
        use akurai_json::Value;
        let pairs = match val {
            Value::Object(p) => p,
            _ => return Err(crate::CoreError::InvalidEntity("expected object for Note".into())),
        };
        let mut n = Note {
            id, title: None, body: String::new(), person_ids: vec![], company_ids: vec![],
            opportunity_ids: vec![], created_at: 0, updated_at: 0, created_by: None,
        };
        for (k, v) in pairs {
            match k.as_str() {
                "title" => n.title = Some(v.as_str().unwrap_or("").to_string()),
                "body" => n.body = v.as_str().unwrap_or("").to_string(),
                "createdAt" => n.created_at = v.as_f64().map(|n| n as i64).unwrap_or(0),
                "updatedAt" => n.updated_at = v.as_f64().map(|n| n as i64).unwrap_or(0),
                _ => {
                    if k == "personIds" {
                        if let Value::Array(arr) = v {
                            n.person_ids = arr.iter().filter_map(|x| x.as_f64().map(|n| n as u64)).collect();
                        }
                    }
                    if k == "companyIds" {
                        if let Value::Array(arr) = v {
                            n.company_ids = arr.iter().filter_map(|x| x.as_f64().map(|n| n as u64)).collect();
                        }
                    }
                    if k == "opportunityIds" {
                        if let Value::Array(arr) = v {
                            n.opportunity_ids = arr.iter().filter_map(|x| x.as_f64().map(|n| n as u64)).collect();
                        }
                    }
                }
            }
        }
        if n.body.is_empty() {
            return Err(crate::CoreError::Validation("note body is required".into()));
        }
        Ok(n)
    }
}

fn actions_json_field(fields: &mut Vec<(String, akurai_json::Value)>, name: &str, ids: &[u64]) {
    use akurai_json::Value;
    let arr: Vec<Value> = ids.iter().map(|id| Value::Int(*id as i64)).collect();
    fields.push((name.into(), Value::Array(arr)));
}
