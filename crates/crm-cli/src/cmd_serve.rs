use akurai_http::{Method, Request, Response, Server};
use crm_api::{build_router, CrmState, Route};
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct Config {
    pub host: String,
    pub port: u16,
    pub dir: PathBuf,
    pub db: String,
}

pub fn run(cfg: Config) -> io::Result<()> {
    let state = CrmState::new(&cfg.db, cfg.dir.clone())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let state = Arc::new(Mutex::new(state));

    let routes = build_router(state);

    let addr = format!("{}:{}", cfg.host, cfg.port);
    let server = Server::bind(&addr)?;
    let local = server.local_addr()?;

    println!("AkurAI-CRM");
    println!("  database: {}", cfg.db);
    println!("  frontend: {}", cfg.dir.display());
    println!("  → http://{local}");

    server.run(move |req: &Request| -> Response { dispatch(&routes, req) })
}

fn dispatch(routes: &[Route], req: &Request) -> Response {
    let mut best: Option<(Response, (usize, usize, usize))> = None;

    for route in routes {
        if !method_matches(route.method, &req.method) {
            continue;
        }
        let pattern = akurai_router::Pattern::parse(route.pattern);
        if pattern.match_path(&req.path).is_some() {
            let score = pattern.specificity();
            if best.as_ref().map_or(true, |(_, b)| score > *b) {
                let resp = (route.handler)(req);
                best = Some((resp, score));
            }
        }
    }

    match best {
        Some((resp, _)) => resp,
        None => Response::not_found(),
    }
}

fn method_matches(pattern: &str, method: &Method) -> bool {
    match method {
        Method::Get => pattern.eq_ignore_ascii_case("GET"),
        Method::Post => pattern.eq_ignore_ascii_case("POST"),
        Method::Put => pattern.eq_ignore_ascii_case("PUT"),
        Method::Patch => pattern.eq_ignore_ascii_case("PATCH"),
        Method::Delete => pattern.eq_ignore_ascii_case("DELETE"),
        Method::Head => pattern.eq_ignore_ascii_case("HEAD"),
        Method::Options => pattern.eq_ignore_ascii_case("OPTIONS"),
        Method::Other(s) => pattern.eq_ignore_ascii_case(s),
    }
}
