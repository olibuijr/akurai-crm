#![forbid(unsafe_code)]

mod error;
mod handlers;
mod router_setup;

pub use error::ApiError;
pub use router_setup::{build_router, CrmState, Route};
