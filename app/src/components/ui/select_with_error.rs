use std::collections::HashMap;

use crate::{
    common::validate_helper::{
        extract_form_field_name, ui_extract_field_errors, validate_field_value,
    },
    components::ui::select_input::SelectInput,
};
use leptos::{leptos_dom::logging::console_log, prelude::*};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use validator::Validate;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn SelectWithError<T>(
    name: String,
    label: String,
    #[prop(optional)] error_class_name: String,
    #[prop(optional)] value: String,
    #[prop(optional)] not_selected_text: String,
    options: Vec<SelectOption>,
    #[prop(into)] on_change: Callback<String>,
    validation_errors: Signal<HashMap<String, Vec<String>>>,
    set_validation_errors: WriteSignal<HashMap<String, Vec<String>>>,
    form_data: T,
) -> impl IntoView
where
    T: Validate + Clone + Debug + Default + Serialize + for<'a> Deserialize<'a> + 'static,
{
    view! {
            <label class="label mx-2" for=name.to_owned()>
                {label}
            </label>
            <SelectInput name={name.to_owned()} not_selected_text=not_selected_text options=options value=value on_change=on_change
                on:input={
                    let field_name = extract_form_field_name(name.to_owned());
                    move |event| {
                        let value = event_target_value(&event);
                        console_log(&format!("select input val={}", value));
                        set_validation_errors.write().insert(field_name.to_owned(), validate_field_value(field_name.to_owned(), value, form_data.clone()));
                    }
                }
            />

        {
            let field_name = extract_form_field_name(name.to_owned());
            move || {
                let errors = ui_extract_field_errors(&field_name, validation_errors);
                errors.map(|list| list.into_iter().map(|msg| view!{ <p class=format!("help is-danger {}", error_class_name)>{msg}</p>}).collect::<Vec<_>>().into_iter().map(|msg| view!{ <p class="help is-danger">{msg}</p>}).collect::<Vec<_>>())
            }
        }

    }
}
