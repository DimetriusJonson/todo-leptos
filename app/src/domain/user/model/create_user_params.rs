use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserParams {
    pub version: usize,

    #[validate(required, regex(path = username_regex(), message="Разрешены только буквы и цифры и не менее 3-х символов."))]
    pub name: Option<String>,
    #[validate(required, length(min = 4))]
    pub password: Option<String>,
}

fn username_regex() -> &'static regex::Regex {
    static RE_POSTAL_CODE: OnceLock<regex::Regex> = OnceLock::new();
    RE_POSTAL_CODE.get_or_init(|| regex::Regex::new("^[А-Яа-яA-Za-z0-9 ]{3,}$").unwrap())
}
