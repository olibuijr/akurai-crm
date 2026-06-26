use crate::types::*;

/// A contact person (individual)
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub id: RecordId,
    pub name: FullName,
    pub email: Option<Email>,
    pub phone: Option<Phone>,
    pub job_title: Option<String>,
    pub company_id: Option<RecordId>,
    pub linkedin_url: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub created_by: Option<RecordId>,
}

impl Person {
    pub fn to_json(&self) -> akurai_json::Value {
        use akurai_json::Value;
        let mut fields = vec![
            ("id".into(), Value::Int(self.id as i64)),
            ("name".into(), Value::Str(self.name.display())),
            ("firstName".into(), Value::Str(self.name.first.clone())),
            ("lastName".into(), Value::Str(self.name.last.clone())),
        ];
        if let Some(ref e) = self.email {
            fields.push(("email".into(), Value::Str(e.address.clone())));
        }
        if let Some(ref p) = self.phone {
            fields.push(("phone".into(), Value::Str(p.number.clone())));
        }
        if let Some(ref jt) = self.job_title {
            fields.push(("jobTitle".into(), Value::Str(jt.clone())));
        }
        if let Some(cid) = self.company_id {
            fields.push(("companyId".into(), Value::Int(cid as i64)));
        }
        if let Some(ref li) = self.linkedin_url {
            fields.push(("linkedinUrl".into(), Value::Str(li.clone())));
        }
        if let Some(ref av) = self.avatar_url {
            fields.push(("avatarUrl".into(), Value::Str(av.clone())));
        }
        fields.push(("createdAt".into(), Value::Int(self.created_at)));
        fields.push(("updatedAt".into(), Value::Int(self.updated_at)));
        Value::Object(fields)
    }

    pub fn from_json(id: RecordId, val: &akurai_json::Value) -> Result<Self, crate::CoreError> {
        use akurai_json::Value;
        let pairs = match val {
            Value::Object(p) => p,
            _ => {
                return Err(crate::CoreError::InvalidEntity(
                    "expected object for Person".into(),
                ))
            }
        };
        let mut p = Person {
            id,
            name: FullName::new("", ""),
            email: None,
            phone: None,
            job_title: None,
            company_id: None,
            linkedin_url: None,
            avatar_url: None,
            created_at: 0,
            updated_at: 0,
            created_by: None,
        };
        for (k, v) in pairs {
            match k.as_str() {
                "firstName" => p.name.first = v.as_str().unwrap_or("").to_string(),
                "lastName" => p.name.last = v.as_str().unwrap_or("").to_string(),
                "email" => {
                    p.email = Some(Email {
                        address: v.as_str().unwrap_or("").to_string(),
                        label: "work".into(),
                    })
                }
                "phone" => {
                    p.phone = Some(Phone {
                        number: v.as_str().unwrap_or("").to_string(),
                        label: "mobile".into(),
                    })
                }
                "jobTitle" => p.job_title = Some(v.as_str().unwrap_or("").to_string()),
                "companyId" => p.company_id = v.as_i64().map(|n| n as u64),
                "linkedinUrl" => p.linkedin_url = Some(v.as_str().unwrap_or("").to_string()),
                "avatarUrl" => p.avatar_url = Some(v.as_str().unwrap_or("").to_string()),
                "createdAt" => p.created_at = v.as_i64().unwrap_or(0),
                "updatedAt" => p.updated_at = v.as_i64().unwrap_or(0),
                _ => {}
            }
        }
        Ok(p)
    }
}
