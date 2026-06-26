use crate::types::*;

/// An activity timeline entry — polymorphic: can link to any entity type.
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineActivity {
    pub id: RecordId,
    pub entity_type: EntityType,
    pub entity_id: RecordId,
    pub action: String, // "created", "updated", "note_added", "stage_changed", etc.
    pub details: Option<String>, // JSON with change details
    pub actor_id: Option<RecordId>,
    pub created_at: Timestamp,
}

impl TimelineActivity {
    pub fn to_json(&self) -> akurai_json::Value {
        use akurai_json::Value;
        let mut fields = vec![
            ("id".into(), Value::Int(self.id as i64)),
            ("entityType".into(), Value::Str(self.entity_type.singular().into())),
            ("entityId".into(), Value::Int(self.entity_id as i64)),
            ("action".into(), Value::Str(self.action.clone())),
        ];
        if let Some(ref d) = self.details {
            fields.push(("details".into(), Value::Str(d.clone())));
        }
        if let Some(a) = self.actor_id {
            fields.push(("actorId".into(), Value::Int(a as i64)));
        }
        fields.push(("createdAt".into(), Value::Int(self.created_at)));
        Value::Object(fields)
    }
}
