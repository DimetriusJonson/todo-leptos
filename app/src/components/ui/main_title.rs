use leptos::prelude::*;

#[component]
pub fn MainTitle(
    title: String,
    #[prop(optional)] class_name: String,
) -> impl IntoView {
    view! {
        <h1 class={format!("title {}", class_name)}>
            {title}
        </h1>
    }
}
