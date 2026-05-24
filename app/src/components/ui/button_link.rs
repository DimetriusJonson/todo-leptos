use leptos::prelude::*;

#[component]
pub fn ButtonLink(
    #[prop(optional)] id: i32,
    label: String,
    href: String,
    #[prop(optional)] class_name: String,
    loading: Option<ReadSignal<bool>>,
) -> impl IntoView {
    let aria_label = label.to_owned();
    view! {
        <a id=id aria-label=aria_label href=href
            class=format!("button is-rounded {}", class_name)
            class:is-loading=move || match loading {
                Some(loading) => loading.get(),
                None => false,
            }>
            {label}
        </a>
    }
}
