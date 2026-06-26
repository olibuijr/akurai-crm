#![forbid(unsafe_code)]

mod auth;
mod error;
mod handlers;
mod router_setup;

pub use auth::session;
pub use error::ApiError;
pub use router_setup::{build_router, CrmState, Route};
