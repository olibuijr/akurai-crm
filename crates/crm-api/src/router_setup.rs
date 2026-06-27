use crate::auth;
use crate::handlers;
use akurai_http::{Request, Response};
use akurai_storage::BTree;
use crm_core::metadata::ObjectMetadata;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// A registered route: HTTP method, URL pattern, and handler closure.
pub struct Route {
    pub method: &'static str,
    pub pattern: &'static str,
    pub handler: Box<dyn Fn(&Request) -> Response + Send + Sync>,
}

/// Shared CRM application state
pub struct CrmState {
    pub db: Arc<Mutex<BTree>>,
    pub objects: Vec<ObjectMetadata>,
    pub frontend_dir: PathBuf,
}

impl CrmState {
    pub fn new(db_path: &str, frontend_dir: PathBuf) -> Result<Self, String> {
        let btree = BTree::open(db_path).map_err(|e| format!("failed to open storage: {e}"))?;
        let objects = ObjectMetadata::standard_objects();
        Ok(Self {
            db: Arc::new(Mutex::new(btree)),
            objects,
            frontend_dir,
        })
    }

    pub fn now() -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }
}

pub fn r(
    state: &'static str,
    p: &'static str,
    h: Box<dyn Fn(&Request) -> Response + Send + Sync>,
) -> Route {
    Route {
        method: state,
        pattern: p,
        handler: h,
    }
}

fn health_route() -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(|_req: &Request| {
        json_response(akurai_json::Value::Object(vec![
            ("app".into(), akurai_json::Value::Str("akurai-crm".into())),
            ("framework".into(), akurai_json::Value::Str("AkurAI-Framework".into())),
            ("framework_version".into(), akurai_json::Value::Str("0.8.2".into())),
            ("version".into(), akurai_json::Value::Str(env!("CARGO_PKG_VERSION").into())),
            ("status".into(), akurai_json::Value::Str("ok".into())),
        ]))
    })
}

/// Build the route table for the CRM API
pub fn build_router(state: Arc<Mutex<CrmState>>) -> Vec<Route> {
    vec![
        r("OPTIONS", "/*", options_handler()),
        r("GET", "/", handlers::static_file_route(Arc::clone(&state))),
        // Auth routes (before catch-all /*)
        r("GET", "/auth/login", auth::login_route(Arc::clone(&state))),
        r(
            "GET",
            "/auth/callback",
            auth::callback_route(Arc::clone(&state)),
        ),
        r(
            "GET",
            "/auth/logout",
            auth::logout_route(Arc::clone(&state)),
        ),
        r("GET", "/api/me", auth::me_route(Arc::clone(&state))),
        r("GET", "/api/health", health_route()),
        r("GET", "/*", handlers::static_file_route(Arc::clone(&state))),
        r(
            "GET",
            "/api/_meta",
            handlers::meta_route(Arc::clone(&state)),
        ),
        r(
            "GET",
            "/api/people",
            handlers::list_route(Arc::clone(&state), "people"),
        ),
        r(
            "GET",
            "/api/people/:id",
            handlers::get_route(Arc::clone(&state), "people"),
        ),
        r(
            "POST",
            "/api/people",
            handlers::create_route(Arc::clone(&state), "people"),
        ),
        r(
            "PUT",
            "/api/people/:id",
            handlers::update_route(Arc::clone(&state), "people"),
        ),
        r(
            "DELETE",
            "/api/people/:id",
            handlers::delete_route(Arc::clone(&state), "people"),
        ),
        r(
            "GET",
            "/api/companies",
            handlers::list_route(Arc::clone(&state), "companies"),
        ),
        r(
            "GET",
            "/api/companies/:id",
            handlers::get_route(Arc::clone(&state), "companies"),
        ),
        r(
            "POST",
            "/api/companies",
            handlers::create_route(Arc::clone(&state), "companies"),
        ),
        r(
            "PUT",
            "/api/companies/:id",
            handlers::update_route(Arc::clone(&state), "companies"),
        ),
        r(
            "DELETE",
            "/api/companies/:id",
            handlers::delete_route(Arc::clone(&state), "companies"),
        ),
        r(
            "GET",
            "/api/opportunities",
            handlers::list_route(Arc::clone(&state), "opportunities"),
        ),
        r(
            "GET",
            "/api/opportunities/:id",
            handlers::get_route(Arc::clone(&state), "opportunities"),
        ),
        r(
            "POST",
            "/api/opportunities",
            handlers::create_route(Arc::clone(&state), "opportunities"),
        ),
        r(
            "PUT",
            "/api/opportunities/:id",
            handlers::update_route(Arc::clone(&state), "opportunities"),
        ),
        r(
            "DELETE",
            "/api/opportunities/:id",
            handlers::delete_route(Arc::clone(&state), "opportunities"),
        ),
        r(
            "GET",
            "/api/tasks",
            handlers::list_route(Arc::clone(&state), "tasks"),
        ),
        r(
            "GET",
            "/api/tasks/:id",
            handlers::get_route(Arc::clone(&state), "tasks"),
        ),
        r(
            "POST",
            "/api/tasks",
            handlers::create_route(Arc::clone(&state), "tasks"),
        ),
        r(
            "PUT",
            "/api/tasks/:id",
            handlers::update_route(Arc::clone(&state), "tasks"),
        ),
        r(
            "DELETE",
            "/api/tasks/:id",
            handlers::delete_route(Arc::clone(&state), "tasks"),
        ),
        r(
            "GET",
            "/api/notes",
            handlers::list_route(Arc::clone(&state), "notes"),
        ),
        r(
            "GET",
            "/api/notes/:id",
            handlers::get_route(Arc::clone(&state), "notes"),
        ),
        r(
            "POST",
            "/api/notes",
            handlers::create_route(Arc::clone(&state), "notes"),
        ),
        r(
            "PUT",
            "/api/notes/:id",
            handlers::update_route(Arc::clone(&state), "notes"),
        ),
        r(
            "DELETE",
            "/api/notes/:id",
            handlers::delete_route(Arc::clone(&state), "notes"),
        ),
        r(
            "GET",
            "/api/search",
            handlers::search_route(Arc::clone(&state)),
        ),
        r(
            "GET",
            "/api/timeline",
            handlers::timeline_route(Arc::clone(&state)),
        ),
    ]
}

/// Upper bound helper for BTree range queries: appends a 0xFF byte so the bound
/// is strictly greater than every key with the given prefix.
pub fn upper_bound(prefix: &[u8]) -> Vec<u8> {
    let mut bound = prefix.to_vec();
    bound.push(0xff);
    bound
}

pub fn json_response(data: akurai_json::Value) -> Response {
    let body = data.to_json();
    Response::ok()
        .with_header("Content-Type", "application/json")
        .with_header("Access-Control-Allow-Origin", "*")
        .with_body("application/json", body.into_bytes())
}

pub fn error_response(status: u16, msg: &str) -> Response {
    let data =
        akurai_json::Value::Object(vec![("error".into(), akurai_json::Value::Str(msg.into()))]);
    let mut resp = json_response(data);
    resp.status = status;
    resp.reason = match status {
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Error",
    }
    .into();
    resp
}

pub fn options_handler() -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(|_req: &Request| {
        Response::new(204)
            .with_header("Access-Control-Allow-Origin", "*")
            .with_header(
                "Access-Control-Allow-Methods",
                "GET, POST, PUT, DELETE, OPTIONS",
            )
            .with_header(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization",
            )
            .with_header("Access-Control-Max-Age", "86400")
    })
}

pub fn not_found(msg: &str) -> Response {
    error_response(404, msg)
}

pub fn bad_request(msg: &str) -> Response {
    error_response(400, msg)
}

pub fn internal_error(msg: &str) -> Response {
    error_response(500, msg)
}
