use crm_core::*;

#[test]
fn test_person_roundtrip() {
    let person = Person {
        id: 1,
        name: FullName::new("Anna", "Jónsdóttir"),
        email: Some(Email {
            address: "anna@test.is".into(),
            label: "work".into(),
        }),
        phone: Some(Phone {
            number: "+354 123 4567".into(),
            label: "mobile".into(),
        }),
        job_title: Some("CEO".into()),
        company_id: Some(1),
        linkedin_url: None,
        avatar_url: None,
        created_at: 1000,
        updated_at: 1000,
        created_by: None,
    };

    let json = person.to_json();
    let parsed = Person::from_json(1, &json).unwrap();
    assert_eq!(parsed.name.first, "Anna");
    assert_eq!(parsed.name.last, "Jónsdóttir");
    assert_eq!(parsed.email.as_ref().unwrap().address, "anna@test.is");
    assert_eq!(parsed.company_id, Some(1));
}

#[test]
fn test_company_validation() {
    let result = Company::from_json(
        1,
        &akurai_json::Value::Object(vec![("name".into(), akurai_json::Value::Str("".into()))]),
    );
    assert!(result.is_err());
}

#[test]
fn test_pipeline_stages() {
    let stages = PipelineStage::all();
    assert_eq!(stages.len(), 7);
    assert_eq!(stages[0].label(), "New");
    assert_eq!(stages[3].label(), "Proposal");
    assert_eq!(stages[5].label(), "Won");
    assert_eq!(stages[6].label(), "Lost");
}

#[test]
fn test_task_status_label() {
    assert_eq!(TaskStatus::Todo.label(), "Todo");
    assert_eq!(TaskStatus::InProgress.label(), "In Progress");
    assert_eq!(TaskStatus::Done.label(), "Done");
}

#[test]
fn test_entity_type_plural() {
    assert_eq!(EntityType::Person.plural(), "people");
    assert_eq!(EntityType::Company.plural(), "companies");
    assert_eq!(EntityType::Opportunity.plural(), "opportunities");
}

#[test]
fn test_opportunity_roundtrip() {
    let opp = Opportunity {
        id: 2,
        name: "Big Deal".into(),
        amount: Some(100000),
        stage: PipelineStage::Negotiation,
        close_date: Some(2000),
        probability: Some(75),
        person_id: Some(1),
        company_id: Some(2),
        created_at: 1000,
        updated_at: 1000,
        created_by: None,
    };

    let json = opp.to_json();
    let parsed = Opportunity::from_json(2, &json).unwrap();
    assert_eq!(parsed.name, "Big Deal");
    assert_eq!(parsed.stage, PipelineStage::Negotiation);
    assert_eq!(parsed.amount, Some(100000));
    assert_eq!(parsed.probability, Some(75));
    assert_eq!(parsed.person_id, Some(1));
    assert_eq!(parsed.company_id, Some(2));
}
