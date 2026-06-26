use akurai_json::Value;
use akurai_storage::BTree;
use std::sync::{Arc, Mutex};

pub struct ToolRegistry {
    db: Arc<Mutex<BTree>>,
}

impl ToolRegistry {
    pub fn new(db: Arc<Mutex<BTree>>) -> Self {
        Self { db }
    }

    /// List all available tools with their schemas
    pub fn list_tools(&self) -> Vec<Value> {
        vec![
            self.tool_def(
                "list_people",
                "List all CRM contacts",
                vec![("limit", "integer", "Maximum number of results", false)],
            ),
            self.tool_def(
                "get_person",
                "Get a person by ID",
                vec![("id", "integer", "Person record ID", true)],
            ),
            self.tool_def(
                "create_person",
                "Create a new person",
                vec![
                    ("firstName", "string", "First name", true),
                    ("lastName", "string", "Last name", true),
                    ("email", "string", "Email address", false),
                    ("jobTitle", "string", "Job title", false),
                    ("companyId", "integer", "Company ID", false),
                ],
            ),
            self.tool_def(
                "list_companies",
                "List all companies",
                vec![("limit", "integer", "Maximum number of results", false)],
            ),
            self.tool_def(
                "get_company",
                "Get a company by ID",
                vec![("id", "integer", "Company record ID", true)],
            ),
            self.tool_def(
                "list_opportunities",
                "List all sales opportunities",
                vec![
                    (
                        "stage",
                        "string",
                        "Filter by stage (new/screening/meeting/proposal/negotiation/won/lost)",
                        false,
                    ),
                    ("limit", "integer", "Maximum number of results", false),
                ],
            ),
            self.tool_def(
                "get_opportunity",
                "Get an opportunity by ID",
                vec![("id", "integer", "Opportunity record ID", true)],
            ),
            self.tool_def(
                "get_pipeline",
                "Get pipeline summary with counts per stage",
                vec![],
            ),
            self.tool_def(
                "search_crm",
                "Search across all CRM entities",
                vec![("query", "string", "Search term", true)],
            ),
            self.tool_def(
                "list_tasks",
                "List tasks",
                vec![
                    (
                        "status",
                        "string",
                        "Filter by status (todo/in_progress/done/cancelled)",
                        false,
                    ),
                    ("limit", "integer", "Maximum number of results", false),
                ],
            ),
        ]
    }

    fn tool_def(
        &self,
        name: &str,
        description: &str,
        params: Vec<(&str, &str, &str, bool)>,
    ) -> Value {
        let mut properties = Vec::new();
        let mut required = Vec::new();
        for (pname, ptype, pdesc, is_required) in params {
            properties.push((
                pname.to_string(),
                Value::Object(vec![
                    ("type".into(), Value::Str(ptype.into())),
                    ("description".into(), Value::Str(pdesc.into())),
                ]),
            ));
            if is_required {
                required.push(Value::Str(pname.to_string()));
            }
        }
        Value::Object(vec![
            ("name".into(), Value::Str(name.into())),
            ("description".into(), Value::Str(description.into())),
            (
                "inputSchema".into(),
                Value::Object(vec![
                    ("type".into(), Value::Str("object".into())),
                    ("properties".into(), Value::Object(properties)),
                    ("required".into(), Value::Array(required)),
                ]),
            ),
        ])
    }

    /// Execute a tool by name with given arguments
    pub fn execute(&self, tool_name: &str, args: &[(String, Value)]) -> Result<Value, String> {
        match tool_name {
            "list_people" => self.list_people(args),
            "get_person" => self.get_person(args),
            "create_person" => self.create_person(args),
            "list_companies" => self.list_companies(args),
            "get_company" => self.get_company(args),
            "list_opportunities" => self.list_opportunities(args),
            "get_opportunity" => self.get_opportunity(args),
            "get_pipeline" => self.get_pipeline(),
            "search_crm" => self.search_crm(args),
            "list_tasks" => self.list_tasks(args),
            _ => Err(format!("unknown tool: {tool_name}")),
        }
    }

    /// Handle a full JSON-RPC MCP request and return response JSON
    pub fn handle_request(&self, body: &str) -> String {
        let req = match akurai_json::parse(body) {
            Ok(v) => v,
            Err(e) => return self.json_rpc_error(None, -32700, &format!("Parse error: {e}")),
        };

        let method = req.get("method").and_then(|v| v.as_str()).unwrap_or("");
        let id = req.get("id").cloned();

        match method {
            "tools/list" => {
                let tools_list = self.list_tools();
                self.json_rpc_result(
                    id,
                    Value::Object(vec![("tools".into(), Value::Array(tools_list))]),
                )
            }
            "tools/call" => {
                let params = match req.get("params") {
                    Some(Value::Object(pairs)) => pairs.clone(),
                    _ => vec![],
                };
                let tool_name = params
                    .iter()
                    .find(|(k, _)| k == "name")
                    .and_then(|(_, v)| v.as_str())
                    .unwrap_or("");
                let arguments: Vec<(String, Value)> = params
                    .iter()
                    .find(|(k, _)| k == "arguments")
                    .and_then(|(_, v)| match v {
                        Value::Object(pairs) => Some(pairs.clone()),
                        _ => None,
                    })
                    .unwrap_or_default();

                match self.execute(tool_name, &arguments) {
                    Ok(result) => self.json_rpc_result(
                        id,
                        Value::Object(vec![(
                            "content".into(),
                            Value::Array(vec![Value::Object(vec![
                                ("type".into(), Value::Str("json".into())),
                                ("text".into(), Value::Str(result.to_json())),
                            ])]),
                        )]),
                    ),
                    Err(e) => self.json_rpc_error(id, -32603, &format!("Tool error: {e}")),
                }
            }
            "tools/get" => {
                let params = match req.get("params") {
                    Some(Value::Object(pairs)) => pairs.clone(),
                    _ => vec![],
                };
                let tool_name = params
                    .iter()
                    .find(|(k, _)| k == "name")
                    .and_then(|(_, v)| v.as_str())
                    .unwrap_or("");
                let tools_list = self.list_tools();
                let tool = tools_list
                    .into_iter()
                    .find(|t| t.get("name").and_then(|v| v.as_str()) == Some(tool_name));
                match tool {
                    Some(t) => self.json_rpc_result(id, Value::Object(vec![("tool".into(), t)])),
                    None => self.json_rpc_error(id, -32602, &format!("Unknown tool: {tool_name}")),
                }
            }
            "initialize" => self.json_rpc_result(
                id,
                Value::Object(vec![
                    ("protocolVersion".into(), Value::Str("2025-03-26".into())),
                    (
                        "capabilities".into(),
                        Value::Object(vec![("tools".into(), Value::Object(vec![]))]),
                    ),
                    (
                        "serverInfo".into(),
                        Value::Object(vec![
                            ("name".into(), Value::Str("akurai-crm-mcp".into())),
                            ("version".into(), Value::Str("0.1.0".into())),
                        ]),
                    ),
                ]),
            ),
            "notifications/initialized" => self.json_rpc_result(id, Value::Null),
            _ => self.json_rpc_error(id, -32601, &format!("Method not found: {method}")),
        }
    }

    fn json_rpc_result(&self, id: Option<Value>, result: Value) -> String {
        let mut resp = Vec::new();
        resp.push(("jsonrpc".into(), Value::Str("2.0".into())));
        resp.push(("result".into(), result));
        if let Some(id_val) = id {
            resp.push(("id".into(), id_val));
        }
        Value::Object(resp).to_json()
    }

    fn json_rpc_error(&self, id: Option<Value>, code: i64, message: &str) -> String {
        let mut resp = Vec::new();
        resp.push(("jsonrpc".into(), Value::Str("2.0".into())));
        resp.push((
            "error".into(),
            Value::Object(vec![
                ("code".into(), Value::Int(code)),
                ("message".into(), Value::Str(message.into())),
            ]),
        ));
        if let Some(id_val) = id {
            resp.push(("id".into(), id_val));
        }
        Value::Object(resp).to_json()
    }

    fn get_arg<'a>(args: &'a [(String, Value)], name: &str) -> Option<&'a Value> {
        args.iter().find(|(k, _)| k == name).map(|(_, v)| v)
    }

    fn scan_collection(&self, collection: &str) -> Result<Vec<Value>, String> {
        let prefix = format!("{}:", collection);
        let start = prefix.as_bytes();
        let end = upper_bound(start);
        let mut db = self.db.lock().map_err(|e| format!("lock: {e}"))?;
        let mut results = Vec::new();
        if let Ok(entries) = db.range(start, &end) {
            for (_, val_bytes) in entries {
                let s = String::from_utf8_lossy(&val_bytes).to_string();
                if let Ok(v) = akurai_json::parse(&s) {
                    results.push(v);
                }
            }
        }
        Ok(results)
    }

    fn get_record(&self, collection: &str, id: u64) -> Result<Option<Value>, String> {
        let mut key = format!("{}:", collection).into_bytes();
        key.extend_from_slice(&id.to_be_bytes());
        let mut db = self.db.lock().map_err(|e| format!("lock: {e}"))?;
        match db.get(&key).map_err(|e| format!("storage: {e}"))? {
            Some(bytes) => {
                let s = String::from_utf8_lossy(&bytes).to_string();
                akurai_json::parse(&s)
                    .map(Some)
                    .map_err(|e| format!("parse: {e}"))
            }
            None => Ok(None),
        }
    }

    fn list_people(&self, args: &[(String, Value)]) -> Result<Value, String> {
        let mut people = self.scan_collection("people")?;
        if let Some(limit_val) = Self::get_arg(args, "limit") {
            if let Some(limit) = limit_val.as_i64() {
                people.truncate(limit as usize);
            }
        }
        Ok(Value::Array(people))
    }

    fn get_person(&self, args: &[(String, Value)]) -> Result<Value, String> {
        let id = Self::get_arg(args, "id")
            .and_then(|v| v.as_i64())
            .ok_or("missing id")?;
        self.get_record("people", id as u64)
            .map(|opt| opt.unwrap_or(Value::Null))
    }

    fn create_person(&self, args: &[(String, Value)]) -> Result<Value, String> {
        let mut db = self.db.lock().map_err(|e| format!("lock: {e}"))?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let counter_key = "people:_counter";
        let next_id = match db
            .get(counter_key.as_bytes())
            .map_err(|e| format!("counter read: {e}"))?
        {
            Some(bytes) => {
                let s = String::from_utf8_lossy(&bytes).to_string();
                match s.parse::<u64>() {
                    Ok(id) if id < u64::MAX => id + 1,
                    _ => return Err("ID space exhausted".into()),
                }
            }
            _ => 1,
        };
        db.insert(counter_key.as_bytes(), next_id.to_string().as_bytes())
            .map_err(|e| format!("counter write: {e}"))?;
        db.commit().map_err(|e| format!("commit: {e}"))?;

        let first = Self::get_arg(args, "firstName")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let last = Self::get_arg(args, "lastName")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut fields = vec![
            ("id".into(), Value::Int(next_id as i64)),
            ("firstName".into(), Value::Str(first.into())),
            ("lastName".into(), Value::Str(last.into())),
            ("name".into(), Value::Str(format!("{} {}", first, last))),
            ("createdAt".into(), Value::Int(now)),
            ("updatedAt".into(), Value::Int(now)),
        ];

        if let Some(email) = Self::get_arg(args, "email").and_then(|v| v.as_str()) {
            fields.push(("email".into(), Value::Str(email.into())));
        }
        if let Some(jt) = Self::get_arg(args, "jobTitle").and_then(|v| v.as_str()) {
            fields.push(("jobTitle".into(), Value::Str(jt.into())));
        }
        if let Some(cid) = Self::get_arg(args, "companyId").and_then(|v| v.as_i64()) {
            fields.push(("companyId".into(), Value::Int(cid)));
        }

        let record = Value::Object(fields);
        let mut key = Vec::from(b"people:" as &[u8]);
        key.extend_from_slice(&next_id.to_be_bytes());
        db.insert(&key, record.to_json().as_bytes())
            .map_err(|e| format!("insert: {e}"))?;
        db.commit().map_err(|e| format!("commit: {e}"))?;

        Ok(record)
    }

    fn list_companies(&self, args: &[(String, Value)]) -> Result<Value, String> {
        let mut companies = self.scan_collection("companies")?;
        if let Some(limit_val) = Self::get_arg(args, "limit") {
            if let Some(limit) = limit_val.as_i64() {
                companies.truncate(limit as usize);
            }
        }
        Ok(Value::Array(companies))
    }

    fn get_company(&self, args: &[(String, Value)]) -> Result<Value, String> {
        let id = Self::get_arg(args, "id")
            .and_then(|v| v.as_i64())
            .ok_or("missing id")?;
        self.get_record("companies", id as u64)
            .map(|opt| opt.unwrap_or(Value::Null))
    }

    fn list_opportunities(&self, args: &[(String, Value)]) -> Result<Value, String> {
        let mut opps = self.scan_collection("opportunities")?;
        if let Some(stage) = Self::get_arg(args, "stage").and_then(|v| v.as_str()) {
            opps.retain(|o| o.get("stage").and_then(|v| v.as_str()) == Some(stage));
        }
        if let Some(limit_val) = Self::get_arg(args, "limit") {
            if let Some(limit) = limit_val.as_i64() {
                opps.truncate(limit as usize);
            }
        }
        Ok(Value::Array(opps))
    }

    fn get_opportunity(&self, args: &[(String, Value)]) -> Result<Value, String> {
        let id = Self::get_arg(args, "id")
            .and_then(|v| v.as_i64())
            .ok_or("missing id")?;
        self.get_record("opportunities", id as u64)
            .map(|opt| opt.unwrap_or(Value::Null))
    }

    fn get_pipeline(&self) -> Result<Value, String> {
        let opps = self.scan_collection("opportunities")?;
        let stages = [
            "new",
            "screening",
            "meeting",
            "proposal",
            "negotiation",
            "won",
            "lost",
        ];
        let stage_counts: Vec<Value> = stages
            .iter()
            .map(|&stage| {
                let count = opps
                    .iter()
                    .filter(|o| o.get("stage").and_then(|v| v.as_str()) == Some(stage))
                    .count();
                let total_amount: i64 = opps
                    .iter()
                    .filter_map(|o| {
                        if o.get("stage").and_then(|v| v.as_str()) == Some(stage) {
                            o.get("amount").and_then(|v| v.as_i64())
                        } else {
                            None
                        }
                    })
                    .sum();
                Value::Object(vec![
                    ("stage".into(), Value::Str(stage.into())),
                    ("count".into(), Value::Int(count as i64)),
                    ("totalAmount".into(), Value::Int(total_amount)),
                ])
            })
            .collect();

        Ok(Value::Object(vec![
            ("pipeline".into(), Value::Array(stage_counts)),
            ("totalDeals".into(), Value::Int(opps.len() as i64)),
        ]))
    }

    fn search_crm(&self, args: &[(String, Value)]) -> Result<Value, String> {
        let query = Self::get_arg(args, "query")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_lowercase();
        if query.is_empty() {
            return Ok(Value::Array(vec![]));
        }

        let mut db = self.db.lock().map_err(|e| format!("lock: {e}"))?;
        let mut results = Vec::new();
        let prefixes = [
            "people:",
            "companies:",
            "opportunities:",
            "tasks:",
            "notes:",
        ];

        for prefix_str in &prefixes {
            let prefix = prefix_str.as_bytes();
            let end = upper_bound(prefix);
            if let Ok(entries) = db.range(prefix, &end) {
                for (_, val_bytes) in entries {
                    let val = String::from_utf8_lossy(&val_bytes).to_string();
                    if val.to_lowercase().contains(&query) {
                        if let Ok(json) = akurai_json::parse(&val) {
                            results.push(Value::Object(vec![
                                (
                                    "entity".into(),
                                    Value::Str(prefix_str.trim_end_matches(':').into()),
                                ),
                                ("record".into(), json),
                            ]));
                        }
                    }
                }
            }
        }

        Ok(Value::Array(results))
    }

    fn list_tasks(&self, args: &[(String, Value)]) -> Result<Value, String> {
        let mut tasks = self.scan_collection("tasks")?;
        if let Some(status) = Self::get_arg(args, "status").and_then(|v| v.as_str()) {
            tasks.retain(|t| t.get("status").and_then(|v| v.as_str()) == Some(status));
        }
        if let Some(limit_val) = Self::get_arg(args, "limit") {
            if let Some(limit) = limit_val.as_i64() {
                tasks.truncate(limit as usize);
            }
        }
        Ok(Value::Array(tasks))
    }
}

/// Upper bound helper for BTree range queries: appends a 0xFF byte so the bound
/// is strictly greater than every key with the given prefix.
fn upper_bound(prefix: &[u8]) -> Vec<u8> {
    let mut bound = prefix.to_vec();
    bound.push(0xff);
    bound
}
