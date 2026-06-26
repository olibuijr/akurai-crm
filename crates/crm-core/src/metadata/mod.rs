//! Dynamic metadata system — objects and fields defined in data, not code.
//! Mirrors Twenty CRM's ObjectMetadata / FieldMetadata pattern.

use akurai_json::Value;

/// Defines an object type in the CRM (e.g., "Person", "Company", custom objects)
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectMetadata {
    pub id: u64,
    pub name_singular: String,
    pub name_plural: String,
    pub label_singular: String,
    pub label_plural: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub fields: Vec<FieldMetadata>,
    pub is_custom: bool,
    pub is_searchable: bool,
}

/// Defines a field on an object
#[derive(Debug, Clone, PartialEq)]
pub struct FieldMetadata {
    pub id: u64,
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
    pub description: Option<String>,
    pub is_required: bool,
    pub is_unique: bool,
    pub is_custom: bool,
    pub default_value: Option<Value>,
    pub options: Vec<FieldOption>,
}

/// Supported field types
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Text,
    Number,
    Boolean,
    Date,
    Currency,
    Email,
    Phone,
    Link,
    Address,
    FullName,
    RichText,
    Select,
    MultiSelect,
    Relation,
    Json,
}

/// A select option
#[derive(Debug, Clone, PartialEq)]
pub struct FieldOption {
    pub label: String,
    pub value: String,
    pub color: Option<String>,
}

impl ObjectMetadata {
    /// Create the standard objects for the CRM
    pub fn standard_objects() -> Vec<Self> {
        vec![
            Self::person_object(),
            Self::company_object(),
            Self::opportunity_object(),
            Self::task_object(),
            Self::note_object(),
        ]
    }

    fn person_object() -> Self {
        Self {
            id: 1,
            name_singular: "person".into(),
            name_plural: "people".into(),
            label_singular: "Person".into(),
            label_plural: "People".into(),
            description: Some("A contact person".into()),
            icon: Some("user".into()),
            fields: vec![
                FieldMetadata { id: 1, name: "name".into(), label: "Name".into(), field_type: FieldType::FullName, description: None, is_required: true, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 2, name: "email".into(), label: "Email".into(), field_type: FieldType::Email, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 3, name: "phone".into(), label: "Phone".into(), field_type: FieldType::Phone, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 4, name: "jobTitle".into(), label: "Job Title".into(), field_type: FieldType::Text, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 5, name: "companyId".into(), label: "Company".into(), field_type: FieldType::Relation, description: Some("Related company".into()), is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 6, name: "linkedinUrl".into(), label: "LinkedIn".into(), field_type: FieldType::Link, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
            ],
            is_custom: false,
            is_searchable: true,
        }
    }

    fn company_object() -> Self {
        Self {
            id: 2,
            name_singular: "company".into(),
            name_plural: "companies".into(),
            label_singular: "Company".into(),
            label_plural: "Companies".into(),
            description: Some("An organization or business".into()),
            icon: Some("building".into()),
            fields: vec![
                FieldMetadata { id: 7, name: "name".into(), label: "Name".into(), field_type: FieldType::Text, description: None, is_required: true, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 8, name: "domainName".into(), label: "Domain".into(), field_type: FieldType::Text, description: None, is_required: false, is_unique: true, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 9, name: "annualRevenue".into(), label: "Annual Revenue".into(), field_type: FieldType::Currency, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 10, name: "employeeCount".into(), label: "Employees".into(), field_type: FieldType::Number, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 11, name: "websiteUrl".into(), label: "Website".into(), field_type: FieldType::Link, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 12, name: "linkedinUrl".into(), label: "LinkedIn".into(), field_type: FieldType::Link, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
            ],
            is_custom: false,
            is_searchable: true,
        }
    }

    fn opportunity_object() -> Self {
        Self {
            id: 3,
            name_singular: "opportunity".into(),
            name_plural: "opportunities".into(),
            label_singular: "Opportunity".into(),
            label_plural: "Opportunities".into(),
            description: Some("A sales deal or pipeline entry".into()),
            icon: Some("trending-up".into()),
            fields: vec![
                FieldMetadata { id: 13, name: "name".into(), label: "Name".into(), field_type: FieldType::Text, description: None, is_required: true, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 14, name: "amount".into(), label: "Amount".into(), field_type: FieldType::Currency, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 15, name: "stage".into(), label: "Stage".into(), field_type: FieldType::Select, description: Some("Pipeline stage".into()), is_required: true, is_unique: false, is_custom: false, default_value: Some(Value::Str("new".into())), options: vec![
                    FieldOption { label: "New".into(), value: "new".into(), color: Some("#94a3b8".into()) },
                    FieldOption { label: "Screening".into(), value: "screening".into(), color: Some("#60a5fa".into()) },
                    FieldOption { label: "Meeting".into(), value: "meeting".into(), color: Some("#818cf8".into()) },
                    FieldOption { label: "Proposal".into(), value: "proposal".into(), color: Some("#a78bfa".into()) },
                    FieldOption { label: "Negotiation".into(), value: "negotiation".into(), color: Some("#f59e0b".into()) },
                    FieldOption { label: "Won".into(), value: "won".into(), color: Some("#22c55e".into()) },
                    FieldOption { label: "Lost".into(), value: "lost".into(), color: Some("#ef4444".into()) },
                ]},
                FieldMetadata { id: 16, name: "closeDate".into(), label: "Close Date".into(), field_type: FieldType::Date, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 17, name: "probability".into(), label: "Probability".into(), field_type: FieldType::Number, description: Some("Win probability (%)".into()), is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 18, name: "personId".into(), label: "Contact".into(), field_type: FieldType::Relation, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 19, name: "companyId".into(), label: "Company".into(), field_type: FieldType::Relation, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
            ],
            is_custom: false,
            is_searchable: true,
        }
    }

    fn task_object() -> Self {
        Self {
            id: 4,
            name_singular: "task".into(),
            name_plural: "tasks".into(),
            label_singular: "Task".into(),
            label_plural: "Tasks".into(),
            description: Some("A to-do item".into()),
            icon: Some("check-square".into()),
            fields: vec![
                FieldMetadata { id: 20, name: "title".into(), label: "Title".into(), field_type: FieldType::Text, description: None, is_required: true, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 21, name: "description".into(), label: "Description".into(), field_type: FieldType::RichText, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 22, name: "status".into(), label: "Status".into(), field_type: FieldType::Select, description: None, is_required: true, is_unique: false, is_custom: false, default_value: Some(Value::Str("todo".into())), options: vec![
                    FieldOption { label: "Todo".into(), value: "todo".into(), color: Some("#94a3b8".into()) },
                    FieldOption { label: "In Progress".into(), value: "in_progress".into(), color: Some("#60a5fa".into()) },
                    FieldOption { label: "Done".into(), value: "done".into(), color: Some("#22c55e".into()) },
                    FieldOption { label: "Cancelled".into(), value: "cancelled".into(), color: Some("#ef4444".into()) },
                ]},
                FieldMetadata { id: 23, name: "dueAt".into(), label: "Due Date".into(), field_type: FieldType::Date, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
            ],
            is_custom: false,
            is_searchable: true,
        }
    }

    fn note_object() -> Self {
        Self {
            id: 5,
            name_singular: "note".into(),
            name_plural: "notes".into(),
            label_singular: "Note".into(),
            label_plural: "Notes".into(),
            description: Some("A rich-text note".into()),
            icon: Some("file-text".into()),
            fields: vec![
                FieldMetadata { id: 24, name: "title".into(), label: "Title".into(), field_type: FieldType::Text, description: None, is_required: false, is_unique: false, is_custom: false, default_value: None, options: vec![] },
                FieldMetadata { id: 25, name: "body".into(), label: "Body".into(), field_type: FieldType::RichText, description: None, is_required: true, is_unique: false, is_custom: false, default_value: None, options: vec![] },
            ],
            is_custom: false,
            is_searchable: true,
        }
    }
}
