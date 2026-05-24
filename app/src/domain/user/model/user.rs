use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User{
    pub id: Option<i64>,
    pub username: Option<String>,
    pub token: Option<String>,
    pub password: Option<String>,
}