use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginParams {
    pub version: usize,

    #[validate(required, length(min = 3))]
    pub name: Option<String>,
    #[validate(required, length(min = 4))]
    pub password: Option<String>,
}