use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Opportunity {
    pub id: RecordId,
    pub name: String,
    pub amount: Option<CurrencyAmount>,
    pub stage: PipelineStage,
    pub close_date: Option<Timestamp>,
    pub probability: Option<u8>, // 0-100
    pub person_id: Option<RecordId>,
    pub company_id: Option<RecordId>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub created_by: Option<RecordId>,
}

impl Opportunity {
    pub fn to_json(&self) -> akurai_json::Value {
        use akurai_json::Value;
        let mut fields = Vec::new();
        fields.push(("id".into(), Value::Int(self.id as i64)));
        fields.push(("name".into(), Value::Str(self.name.clone())));
        if let Some(a) = self.amount {
            fields.push(("amount".into(), Value::Int(a)));
        }
        fields.push(("stage".into(), Value::Str(self.stage.label().into())));
        if let Some(cd) = self.close_date {
            fields.push(("closeDate".into(), Value::Int(cd)));
        }
        if let Some(p) = self.probability {
            fields.push(("probability".into(), Value::Int(p as i64)));
        }
        if let Some(pid) = self.person_id {
            fields.push(("personId".into(), Value::Int(pid as i64)));
        }
        if let Some(cid) = self.company_id {
            fields.push(("companyId".into(), Value::Int(cid as i64)));
        }
        fields.push(("createdAt".into(), Value::Int(self.created_at)));
        fields.push(("updatedAt".into(), Value::Int(self.updated_at)));
        Value::Object(fields)
    }

    pub fn from_json(id: RecordId, val: &akurai_json::Value) -> Result<Self, crate::CoreError> {
        use akurai_json::Value;
        let pairs = match val {
            Value::Object(p) => p,
            _ => return Err(crate::CoreError::InvalidEntity("expected object for Opportunity".into())),
        };
        let mut o = Opportunity {
            id, name: String::new(), amount: None, stage: PipelineStage::New,
            close_date: None, probability: None, person_id: None, company_id: None,
            created_at: 0, updated_at: 0, created_by: None,
        };
        for (k, v) in pairs {
            match k.as_str() {
                "name" => o.name = v.as_str().unwrap_or("").to_string(),
                "amount" => o.amount = v.as_f64().map(|n| n as i64),
                "stage" => {
                    let s = v.as_str().unwrap_or("new");
                    o.stage = match s.to_lowercase().as_str() {
                        "screening" => PipelineStage::Screening,
                        "meeting" => PipelineStage::Meeting,
                        "proposal" => PipelineStage::Proposal,
                        "negotiation" => PipelineStage::Negotiation,
                        "won" => PipelineStage::Won,
                        "lost" => PipelineStage::Lost,
                        _ => PipelineStage::New,
                    };
                }
                "closeDate" => o.close_date = v.as_f64().map(|n| n as i64),
                "probability" => o.probability = v.as_f64().map(|n| n as u8),
                "personId" => o.person_id = v.as_f64().map(|n| n as u64),
                "companyId" => o.company_id = v.as_f64().map(|n| n as u64),
                "createdAt" => o.created_at = v.as_f64().map(|n| n as i64).unwrap_or(0),
                "updatedAt" => o.updated_at = v.as_f64().map(|n| n as i64).unwrap_or(0),
                _ => {}
            }
        }
        if o.name.is_empty() {
            return Err(crate::CoreError::Validation("opportunity name is required".into()));
        }
        Ok(o)
    }
}
