use leptos::prelude::*;

#[component]
pub fn Checkbox(
    name: String,
    title: String,
    value: bool,
    #[prop(optional)] class_name: String,
) -> impl IntoView {
    view! {
        <label class=format!("b-checkbox checkbox {}", class_name)>
            <input type="checkbox" name=name.to_owned() checked=value/>
            <span class="check is-warning" title=title></span>
        </label>
    }
}
