use crate::router_setup::{json_response, CrmState};
use akurai_http::{Request, Response};
use akurai_json::Value;
use std::sync::{Arc, Mutex};

pub fn meta_route(state: Arc<Mutex<CrmState>>) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |_req: &Request| {
        let state = state.lock().unwrap();
        let objects: Vec<Value> = state.objects.iter().map(|obj| {
            let fields: Vec<Value> = obj.fields.iter().map(|f| {
                Value::Object(vec![
                    ("id".into(), Value::Int(f.id as i64)),
                    ("name".into(), Value::Str(f.name.clone())),
                    ("label".into(), Value::Str(f.label.clone())),
                    ("type".into(), Value::Str(format!("{:?}", f.field_type).to_lowercase())),
                    ("required".into(), Value::Bool(f.is_required)),
                    ("unique".into(), Value::Bool(f.is_unique)),
                    ("description".into(), Value::Str(f.description.clone().unwrap_or_default())),
                ])
            }).collect();

            Value::Object(vec![
                ("id".into(), Value::Int(obj.id as i64)),
                ("nameSingular".into(), Value::Str(obj.name_singular.clone())),
                ("namePlural".into(), Value::Str(obj.name_plural.clone())),
                ("labelSingular".into(), Value::Str(obj.label_singular.clone())),
                ("labelPlural".into(), Value::Str(obj.label_plural.clone())),
                ("description".into(), Value::Str(obj.description.clone().unwrap_or_default())),
                ("icon".into(), Value::Str(obj.icon.clone().unwrap_or_default())),
                ("isCustom".into(), Value::Bool(obj.is_custom)),
                ("isSearchable".into(), Value::Bool(obj.is_searchable)),
                ("fields".into(), Value::Array(fields)),
            ])
        }).collect();

        json_response(Value::Object(vec![
            ("version".into(), Value::Str("0.1.0".into())),
            ("objects".into(), Value::Array(objects)),
        ]))
    })
}
