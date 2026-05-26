use leptos::prelude::*;

use crate::components::ui::select_input::SelectInput;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn SelectWithLabel(
    name: String,
    label: String,
    #[prop(optional)] select_class_name: String,
    #[prop(optional)] value: String,
    #[prop(optional)] not_selected_text: String,
    options: Vec<SelectOption>,
    #[prop(into)] on_change: Callback<String>,
    errors: impl Fn() -> Option<Vec<String>> + Send + Sync + 'static,
) -> impl IntoView {
    view! {
            <label class="label mx-2" for=name.to_owned()>
                {label}
            </label>
            <SelectInput name=name class_name=select_class_name not_selected_text=not_selected_text options=options value=value on_change=on_change />
            { move || errors().map(|list| list.into_iter().map(|msg| view!{ <p class="help is-danger">{msg}</p>}).collect::<Vec<_>>()) }
    }
}
