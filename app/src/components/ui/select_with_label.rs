use std::collections::HashMap;
use leptos::prelude::*;
use crate::components::ui::select_input::SelectInput;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn SelectWithLabel(
    name: String,
    label: String,
    #[prop(optional)] value: String,
    #[prop(optional)] not_selected_text: String,
    options: Vec<SelectOption>,
    #[prop(into)] on_change: Callback<String>,
    errors: Signal<HashMap<String, Vec<String>>>,
    error_path: &'static str,
) -> impl IntoView {
    view! {
        <label class="label mx-2" for=name.to_owned()>
            {label}
        </label>
        <SelectInput name=name not_selected_text=not_selected_text options=options value=value on_change=on_change />

        <Show when=move || { errors.read().contains_key(error_path) }>
            <div class="px-4">
                <For each=move || errors.read().get(error_path).unwrap().clone() key=|error| error.clone() let(error)>
                    <p class="help is-danger">{error}</p>
                </For>
            </div>
        </Show>
    }
}
