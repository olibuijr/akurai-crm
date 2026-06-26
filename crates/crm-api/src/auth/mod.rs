pub mod idp_client;
pub mod session;

use crate::router_setup::{internal_error, json_response, CrmState};
use akurai_http::{Request, Response};
use akurai_json::Value;
use std::sync::{Arc, Mutex};

/// GET /auth/login — redirect to IDP /authorize
pub fn login_route(
    state: Arc<Mutex<CrmState>>,
) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    let _ = state; // unused, kept for signature consistency
    Box::new(move |_req: &Request| {
        let client_id = "d5c002f01d1dff402d01439fe3b37918";
        let redirect_uri = "https://akurai-crm.olibuijr.com/auth/callback";
        let state_token = format!(
            "crm_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );

        let location = format!(
            "https://auth.olibuijr.com/authorize\
             ?client_id={client_id}\
             &redirect_uri={redirect_uri}\
             &response_type=code\
             &scope=openid+profile+email+groups\
             &state={state_token}"
        );

        let cookie =
            format!("crm_oauth_state={state_token}; Path=/; HttpOnly; SameSite=Lax; Max-Age=600");

        let mut resp = Response::ok();
        resp.status = 302;
        resp.reason = "Found".to_string();
        resp = resp.with_header("Location", &location);
        resp = resp.with_header("Set-Cookie", &cookie);
        resp
    })
}

/// GET /auth/callback — exchange code, create session, redirect to dashboard
pub fn callback_route(
    state: Arc<Mutex<CrmState>>,
) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let query = req.query.as_deref().unwrap_or("");
        let mut code = String::new();
        let mut returned_state = String::new();

        for pair in query.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                match key {
                    "code" => code = value.to_string(),
                    "state" => returned_state = value.to_string(),
                    _ => {}
                }
            }
        }

        if code.is_empty() || returned_state.is_empty() {
            return auth_redirect_error();
        }

        match idp_client::exchange_code(&code, &returned_state) {
            Ok(userinfo) => {
                let session_id = {
                    let state_lock = match state.lock() {
                        Ok(s) => s,
                        Err(_) => return internal_error("lock poisoned"),
                    };
                    let mut db = match state_lock.db.lock() {
                        Ok(db) => db,
                        Err(_) => return internal_error("db lock poisoned"),
                    };
                    session::create(&mut db, &userinfo.sub, &userinfo.email)
                };

                let cookie =
                    format!("crm_session={session_id}; Path=/; HttpOnly; SameSite=Lax; Max-Age=86400");

                let mut resp = Response::ok();
                resp.status = 302;
                resp.reason = "Found".to_string();
                resp = resp.with_header("Location", "/dashboard.html");
                resp = resp.with_header("Set-Cookie", &cookie);
                resp
            }
            Err(_) => auth_redirect_error(),
        }
    })
}

/// GET /api/me — return current user info from session
pub fn me_route(
    state: Arc<Mutex<CrmState>>,
) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        let session_id = session::extract_session_id(req);
        let user = match session_id {
            Some(ref sid) => {
                let state_lock = match state.lock() {
                    Ok(s) => s,
                    Err(_) => return internal_error("lock poisoned"),
                };
                let mut db = match state_lock.db.lock() {
                    Ok(db) => db,
                    Err(_) => return internal_error("db lock poisoned"),
                };
                session::get_user(&mut db, sid)
            }
            None => None,
        };

        match user {
            Some(u) => json_response(Value::Object(vec![
                ("authenticated".into(), Value::Bool(true)),
                ("sub".into(), Value::Str(u.sub)),
                ("email".into(), Value::Str(u.email)),
            ])),
            None => json_response(Value::Object(vec![
                ("authenticated".into(), Value::Bool(false)),
            ])),
        }
    })
}

/// GET /auth/logout — delete session and clear cookie
pub fn logout_route(
    state: Arc<Mutex<CrmState>>,
) -> Box<dyn Fn(&Request) -> Response + Send + Sync> {
    Box::new(move |req: &Request| {
        if let Some(sid) = session::extract_session_id(req) {
            if let Ok(state_lock) = state.lock() {
                if let Ok(mut db) = state_lock.db.lock() {
                    let key = format!("_session:{sid}");
                    let _ = db.delete(key.as_bytes());
                    let _ = db.commit();
                }
            }
        }

        let cookie = "crm_session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0".to_string();

        let mut resp = Response::ok();
        resp.status = 302;
        resp.reason = "Found".to_string();
        resp = resp.with_header("Location", "/");
        resp = resp.with_header("Set-Cookie", &cookie);
        resp
    })
}

fn auth_redirect_error() -> Response {
    let mut resp = Response::ok();
    resp.status = 302;
    resp.reason = "Found".to_string();
    resp = resp.with_header("Location", "/?auth_error=1");
    resp
}
