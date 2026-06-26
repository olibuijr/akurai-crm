use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Company {
    pub id: RecordId,
    pub name: String,
    pub domain_name: Option<String>,
    pub address: Option<Address>,
    pub annual_revenue: Option<CurrencyAmount>,
    pub employee_count: Option<u32>,
    pub linkedin_url: Option<String>,
    pub website_url: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub created_by: Option<RecordId>,
}

impl Company {
    pub fn to_json(&self) -> akurai_json::Value {
        use akurai_json::Value;
        let mut fields = Vec::new();
        fields.push(("id".into(), Value::Int(self.id as i64)));
        fields.push(("name".into(), Value::Str(self.name.clone())));
        if let Some(ref d) = self.domain_name {
            fields.push(("domainName".into(), Value::Str(d.clone())));
        }
        if let Some(ref a) = self.address {
            fields.push((
                "address".into(),
                Value::Str(format!("{}, {}, {} {}", a.street, a.city, a.state, a.zip)),
            ));
        }
        if let Some(r) = self.annual_revenue {
            fields.push(("annualRevenue".into(), Value::Int(r)));
        }
        if let Some(e) = self.employee_count {
            fields.push(("employeeCount".into(), Value::Int(e as i64)));
        }
        if let Some(ref li) = self.linkedin_url {
            fields.push(("linkedinUrl".into(), Value::Str(li.clone())));
        }
        if let Some(ref w) = self.website_url {
            fields.push(("websiteUrl".into(), Value::Str(w.clone())));
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
                    "expected object for Company".into(),
                ))
            }
        };
        let mut c = Company {
            id,
            name: String::new(),
            domain_name: None,
            address: None,
            annual_revenue: None,
            employee_count: None,
            linkedin_url: None,
            website_url: None,
            created_at: 0,
            updated_at: 0,
            created_by: None,
        };
        for (k, v) in pairs {
            match k.as_str() {
                "name" => c.name = v.as_str().unwrap_or("").to_string(),
                "domainName" => c.domain_name = Some(v.as_str().unwrap_or("").to_string()),
                "annualRevenue" => c.annual_revenue = v.as_i64(),
                "employeeCount" => c.employee_count = v.as_i64().map(|n| n as u32),
                "linkedinUrl" => c.linkedin_url = Some(v.as_str().unwrap_or("").to_string()),
                "websiteUrl" => c.website_url = Some(v.as_str().unwrap_or("").to_string()),
                "createdAt" => c.created_at = v.as_i64().unwrap_or(0),
                "updatedAt" => c.updated_at = v.as_i64().unwrap_or(0),
                _ => {}
            }
        }
        if c.name.is_empty() {
            return Err(crate::CoreError::Validation(
                "company name is required".into(),
            ));
        }
        Ok(c)
    }
}
