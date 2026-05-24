use std::borrow::Cow;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;

use leptos::server_fn::error::ServerFnErrorErr;
use serde::{Deserialize, Serialize};
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiError {
    UnAuthorized(String),
    Validation(ValidationErrors),
    Network(String),
    ServerFn(ServerFnErrorErr),
}

impl ApiError {
    pub fn validation_field(
        field: &'static str,
        code: &'static str,
        message: &'static str,
    ) -> Self {
        let mut errors = ValidationErrors::new();
        errors.add(field, ValidationError::new(code).with_message(Cow::Borrowed(message)));
        return Self::Validation(errors);
    }

    pub fn validation(validation_errors: ValidationErrors) -> Self {
        return Self::Validation(Self::transform_validation_errors(validation_errors));
    }

    fn transform_validation_errors(validation_errors: ValidationErrors) -> ValidationErrors {
        let mut errors_map = validation_errors.0.clone();
        for (key, kind) in validation_errors.0 {
            match kind {
                ValidationErrorsKind::Struct(validation_errors) => {
                    errors_map.insert(
                        key,
                        ValidationErrorsKind::Struct(Box::new(Self::transform_validation_errors(
                            *validation_errors,
                        ))),
                    );
                }
                ValidationErrorsKind::List(btree_map) => {
                    let mut list_errors_map: BTreeMap<usize, Box<ValidationErrors>> =
                        BTreeMap::new();
                    for (i, validation_errors) in btree_map {
                        list_errors_map.insert(
                            i,
                            Box::new(Self::transform_validation_errors(*validation_errors)),
                        );
                    }

                    errors_map.insert(key, ValidationErrorsKind::List(list_errors_map));
                }
                ValidationErrorsKind::Field(validation_errors) => {
                    let mut errors: Vec<ValidationError> = Vec::new();
                    for field_err in validation_errors {
                        let mut new_field = field_err.clone();
                        new_field.message = Self::transform_error_message(field_err);
                        errors.push(new_field);
                    }
                    errors_map.insert(key, ValidationErrorsKind::Field(errors));
                }
            }
        }
        ValidationErrors(errors_map)
    }

    fn transform_error_message(field_err: ValidationError) -> Option<Cow<'static, str>> {
        if field_err.message.is_some() {
            return field_err.message;
        }

        let params = field_err.params;
        let min = params.get("min");
        let max = params.get("max");

        match (field_err.code.as_ref(), min, max) {
            ("required", ..) => Some(Cow::Borrowed("Обязательно для заполнения")),
            ("length", Some(min), Some(max)) => {
                Some(Cow::Owned(format!("Длина от {} до {} символов", min, max)))
            }
            ("length", Some(min), None) => {
                Some(Cow::Owned(format!("Длина минимум {} символа", min)))
            }
            ("length", None, Some(max)) => {
                Some(Cow::Owned(format!("Длина максимум {} символа", max)))
            }
            _ => field_err.message,
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnAuthorized(msg) => write!(f, "Пользователь не авторизован. {}", msg),
            Self::Network(msg) => write!(f, "Ошибка запроса: {}.", msg),
            Self::Validation(errors) => {
                write!(f, "{}", serde_json::to_string(&errors).expect("Failed serialize error!"))
            }
            Self::ServerFn(err) => write!(f, "Ошибка сервера: {}", err),
        }
    }
}

impl Error for ApiError {}
