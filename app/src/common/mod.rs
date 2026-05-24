pub mod validate_helper;
pub mod app_state;
pub mod errors;
pub mod security_context;
pub mod api_error;

#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
pub type DbPool = Pool<Sqlite>; 
