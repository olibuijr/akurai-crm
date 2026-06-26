use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: RecordId,
    pub title: String,
    pub description: Option<String>, // rich text / markdown
    pub status: TaskStatus,
    pub due_at: Option<Timestamp>,
    pub assignee_id: Option<RecordId>, // person or user
    pub person_ids: Vec<RecordId>,   // linked people
    pub company_ids: Vec<RecordId>,  // linked companies
    pub opportunity_ids: Vec<RecordId>, // linked opps
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub created_by: Option<RecordId>,
}

impl Task {
    pub fn to_json(&self) -> akurai_json::Value {
        use akurai_json::Value;
        let mut fields = Vec::new();
        fields.push(("id".into(), Value::Int(self.id as i64)));
        fields.push(("title".into(), Value::Str(self.title.clone())));
        if let Some(ref d) = self.description {
            fields.push(("description".into(), Value::Str(d.clone())));
        }
        fields.push(("status".into(), Value::Str(self.status.label().into())));
        if let Some(d) = self.due_at {
            fields.push(("dueAt".into(), Value::Int(d)));
        }
        if let Some(a) = self.assignee_id {
            fields.push(("assigneeId".into(), Value::Int(a as i64)));
        }
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
            _ => return Err(crate::CoreError::InvalidEntity("expected object for Task".into())),
        };
        let mut t = Task {
            id, title: String::new(), description: None, status: TaskStatus::Todo,
            due_at: None, assignee_id: None, person_ids: vec![], company_ids: vec![],
            opportunity_ids: vec![], created_at: 0, updated_at: 0, created_by: None,
        };
        for (k, v) in pairs {
            match k.as_str() {
                "title" => t.title = v.as_str().unwrap_or("").to_string(),
                "description" => t.description = Some(v.as_str().unwrap_or("").to_string()),
                "status" => {
                    let s = v.as_str().unwrap_or("todo");
                    t.status = match s.to_lowercase().as_str() {
                        "in_progress" | "in-progress" | "in progress" => TaskStatus::InProgress,
                        "done" => TaskStatus::Done,
                        "cancelled" => TaskStatus::Cancelled,
                        _ => TaskStatus::Todo,
                    };
                }
                "dueAt" => t.due_at = v.as_f64().map(|n| n as i64),
                "assigneeId" => t.assignee_id = v.as_f64().map(|n| n as u64),
                "createdAt" => t.created_at = v.as_f64().map(|n| n as i64).unwrap_or(0),
                "updatedAt" => t.updated_at = v.as_f64().map(|n| n as i64).unwrap_or(0),
                _ => {
                    if k == "personIds" {
                        if let Value::Array(arr) = v {
                            t.person_ids = arr.iter().filter_map(|x| x.as_f64().map(|n| n as u64)).collect();
                        }
                    }
                    if k == "companyIds" {
                        if let Value::Array(arr) = v {
                            t.company_ids = arr.iter().filter_map(|x| x.as_f64().map(|n| n as u64)).collect();
                        }
                    }
                    if k == "opportunityIds" {
                        if let Value::Array(arr) = v {
                            t.opportunity_ids = arr.iter().filter_map(|x| x.as_f64().map(|n| n as u64)).collect();
                        }
                    }
                }
            }
        }
        if t.title.is_empty() {
            return Err(crate::CoreError::Validation("task title is required".into()));
        }
        Ok(t)
    }
}

fn actions_json_field(fields: &mut Vec<(String, akurai_json::Value)>, name: &str, ids: &[u64]) {
    use akurai_json::Value;
    let arr: Vec<Value> = ids.iter().map(|id| Value::Int(*id as i64)).collect();
    fields.push((name.into(), Value::Array(arr)));
}
