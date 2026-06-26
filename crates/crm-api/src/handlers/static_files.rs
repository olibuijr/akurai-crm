use crate::router_setup::{internal_error, not_found, CrmState};
use akurai_http::{Request, Response};
use std::sync::{Arc, Mutex};

pub fn static_file_route(state: Arc<Mutex<CrmState>>) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let path = &req.path;
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return internal_error("lock poisoned"),
        };
        let frontend = &state.frontend_dir;

        let file_path = if path == "/" {
            frontend.join("index.html")
        } else {
            let clean = path.trim_start_matches('/');
            frontend.join(clean)
        };

        let canonical = std::fs::canonicalize(&file_path).ok();
        let frontend_canonical = std::fs::canonicalize(frontend).unwrap_or_else(|_| std::path::PathBuf::from("/nonexistent"));
        let is_safe = canonical.as_ref().is_some_and(|c| c.starts_with(&frontend_canonical));

        if !is_safe {
            return not_found("invalid path");
        }

        if !file_path.exists() || !file_path.is_file() {
            let index = frontend.join("index.html");
            return match std::fs::read_to_string(&index) {
                Ok(body) => Response::ok()
                    .with_header("Content-Type", "text/html; charset=utf-8")
                    .with_body("text/html; charset=utf-8", body.into_bytes()),
                Err(_) => not_found("file not found"),
            };
        }

        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let mime = match ext {
            "html" => "text/html; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "js" => "application/javascript; charset=utf-8",
            "json" => "application/json",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "svg" => "image/svg+xml",
            "ico" => "image/x-icon",
            "woff2" => "font/woff2",
            _ => "application/octet-stream",
        };

        match std::fs::read(&file_path) {
            Ok(bytes) => Response::ok()
                .with_header("Content-Type", mime)
                .with_body(mime, bytes),
            Err(_) => not_found("file not found"),
        }
    })
}
