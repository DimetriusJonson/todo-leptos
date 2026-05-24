use std::collections::HashMap;

use leptos::prelude::*;

#[component]
pub fn TextWithError(
    name: String,
    placeholder: String,
    input_type: String,
    #[prop(optional)] value: String,
    errors: Signal<HashMap<String, Vec<String>>>,
    error_path: &'static str,
) -> impl IntoView {
    view! {
            <div class="control">
                <input
                    class={"input"}
                    class:is-danger=move || false
                    type=input_type
                    id=name.to_owned()
                    name=name.to_owned()
                    value=value
                    placeholder=placeholder
                />
            </div>

            <Show
                when=move || {errors.read().contains_key(error_path)}
                fallback=|| view!{ <p></p> }>

                    <For
                        each=move || errors.read().get(error_path).unwrap().clone()
                        key=|error| error.clone()
                        let(error)>
                        <p class="help is-danger">{error}</p>
                    </For>

            </Show>
        }
}
