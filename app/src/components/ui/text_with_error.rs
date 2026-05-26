use leptos::prelude::*;

#[component]
pub fn TextWithError(
    name: String,
    placeholder: String,
    input_type: String,
    #[prop(optional)] value: String,
    errors: impl Fn() -> Option<Vec<String>> + Send + Sync + 'static,
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

        { move || errors().map(|list| list.into_iter().map(|msg| view!{ <p class="px-4 help is-danger">{msg}</p>}).collect::<Vec<_>>()) }
    }
}
