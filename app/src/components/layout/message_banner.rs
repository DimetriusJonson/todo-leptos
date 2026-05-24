use std::time::Duration;

use leptos::prelude::*;
use web_sys::HtmlButtonElement;

#[derive(Debug, Clone, Default)]
struct MessageBannerItem {
    pub id: String,
    pub msg: String,
    pub kind: String,
}

#[derive(Clone, Copy)]
pub struct Messages(RwSignal<Vec<MessageBannerItem>>);

#[component]
pub fn MessageBanner() -> impl IntoView {
    let messages = Messages(RwSignal::new(Vec::<MessageBannerItem>::new()));
    provide_context(messages);

    view! {
        <div
            class="has-text-centered py-3"
            style:position="fixed"
            style:left="0"
            style:bottom="1.5rem"
            style:width="100%"
            style:z-index="1000"
        >
            {move || messages.0.get().into_iter()
                .map(|msg| view! {
                <p class="field">
                    <span class={format!("tag is-medium {}", msg_style(&msg))}>
                        {msg.msg.to_owned()}
                        <button
                            aria-label="x"
                            class="delete is-small"
                            id={format!("m_{}", msg.id)}
                            on:click={move |event| {
                                let id_str = event_target::<HtmlButtonElement>(&event).id().to_string();
                                if let Some(pos) = id_str.find('_') {
                                    let id = &id_str[pos + 1..];
                                    remove_message(id, messages);
                                }
                            }}
                        ></button>
                    </span>
                </p>
                }).collect::<Vec<_>>()}
        </div>
    }
}

pub fn show_info(msg: String, messages: Messages) {
    show_message(msg, "INFO".to_string(), Duration::from_millis(5000), messages);
}

pub fn show_error(msg: String, messages: Messages) {
    show_message(msg, "ERROR".to_string(), Duration::from_millis(30000), messages);
}

fn show_message(msg: String, kind: String, active_time: Duration, messages: Messages) {
    use uuid::Uuid;

    let id = Uuid::new_v4().to_string();
    messages.0.write().push(MessageBannerItem {
        id: id.to_owned(),
        msg,
        kind,
    });

    set_timeout(
        move || {
            remove_message(&id, messages);
        },
        active_time,
    );
}

pub fn remove_message(id: &str, messages: Messages) {
    let mut new_list = messages.0.get();
    new_list.retain(|m| m.id != id);
    messages.0.set(new_list);
}

fn msg_style(msg: &MessageBannerItem) -> String {
    if msg.kind == "INFO" {
        return "is-primary".to_owned();
    }

    "is-danger".to_owned()
}
