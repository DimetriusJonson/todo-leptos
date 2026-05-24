use std::collections::HashMap;

use leptos::reactive::traits::Read;
use leptos::reactive::wrappers::read::Signal;
use leptos::server_fn::ServerFnError;
use serde_json::Value;

pub fn ui_build_validation_errors<T>(
    error: &Option<Result<T, ServerFnError>>,
) -> HashMap<String, Vec<String>> {
    if let Some(Err(ServerFnError::ServerError(msg))) = error {
        match serde_json::from_str::<Value>(msg) {
            Ok(value) => {
                if let Some(value_obj) = value.as_object() {
                    let mut map = HashMap::new();
                    for (field_name, field_errors_val) in value_obj.iter() {
                        if let Value::Array(field_errors) = field_errors_val {
                            map.insert(
                                field_name.to_owned(),
                                field_errors
                                    .iter()
                                    .map(|v| {
                                        v.get("message").unwrap().as_str().unwrap().to_string()
                                    })
                                    .collect(),
                            );
                        };
                    }
                    return map;
                }
            }
            Err(_err) => {
                return HashMap::from([("common_error".to_owned(), vec![msg.to_owned()])]);
            }
        }
    }

    HashMap::new()
}

pub fn ui_build_common_error(errors: Signal<HashMap<String, Vec<String>>>) -> String {
    match errors.read().get("common_error") {
        Some(v) => v.first().unwrap_or(&"".to_owned()).to_owned(),
        None => "".to_owned(),
    }
}
