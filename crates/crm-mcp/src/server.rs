use crate::tools::ToolRegistry;
use akurai_http::{Method, Request, Response};
use akurai_storage::BTree;
use std::sync::{Arc, Mutex};

pub struct McpConfig {
    pub host: String,
    pub port: u16,
    pub db_path: String,
}

pub struct McpServer {
    config: McpConfig,
    tools: Arc<ToolRegistry>,
}

impl McpServer {
    pub fn new(config: McpConfig) -> Self {
        let btree = BTree::open(&config.db_path).expect("failed to open MCP database");
        let tools = ToolRegistry::new(Arc::new(Mutex::new(btree)));
        Self {
            config,
            tools: Arc::new(tools),
        }
    }

    /// Handle an MCP JSON-RPC request
    pub fn handle_request(&self, body: &str) -> String {
        self.tools.handle_request(body)
    }

    /// Start the MCP HTTP server
    pub fn serve(&self) -> Result<(), String> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        println!("🔌 MCP server listening on http://{addr}/mcp");

        let tools = Arc::clone(&self.tools);
        let handler = move |req: &Request| -> Response {
            if req.path != "/mcp" {
                return Response::new(404).with_text("Not found");
            }
            if req.method != Method::Post {
                return Response::new(405).with_text("Method not allowed");
            }

            let body = req.body_str();
            let result = tools.handle_request(body.as_ref());

            Response::ok()
                .with_header("Access-Control-Allow-Origin", "*")
                .with_body("application/json", result.into_bytes())
        };

        let server = akurai_http::Server::bind(&addr)
            .map_err(|e| format!("failed to bind MCP server: {e}"))?;
        server.run(handler).map_err(|e| format!("server error: {e}"))
    }
}
