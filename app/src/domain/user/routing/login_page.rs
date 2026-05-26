use std::collections::HashMap;

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::common::validate_helper::{
    ui_build_common_error, ui_build_validation_errors, ui_extract_field_errors,
};
use crate::components::layout::message_banner::{Messages, show_info};
use crate::components::ui::button::Button;
use crate::components::ui::main_title::MainTitle;
use crate::components::ui::text_with_error::TextWithError;
use crate::domain::user::user_services::Login;

#[component]
pub fn LoginPage() -> impl IntoView {
    let login = ServerAction::<Login>::new();

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    let query_map = use_query_map();
    let def_user_name = move || query_map.with(|m| m.get("defUserName"));

    let validation_errors: Signal<HashMap<String, Vec<String>>> =
        Signal::derive(move || login.value().with(ui_build_validation_errors));
    let common_error = move || ui_build_common_error(validation_errors);

    Effect::new(move |_| {
        if let Some(Ok(_)) = login.value().get() {
            show_info("Вы вошли!".to_owned(), messages);
            login.clear();
        }
    });

    view! {
        <div class="container p-4">
            <MainTitle title="Вход в систему".to_owned() />

            <ActionForm action=login>
                <div class="help is-danger is-size-5 py-4">{common_error}</div>

                <fieldset disabled=login.pending()>
                    <div class="field">
                        { move || view! {
                                <TextWithError input_type="text".to_owned() name="params[name]".to_owned()
                                    placeholder="Имя пользователя".to_owned()
                                    errors=move || ui_extract_field_errors("name", validation_errors)
                                    value={def_user_name().unwrap_or_default()}
                                />
                            }
                        }
                    </div>

                    <div class="field">
                        <TextWithError input_type="password".to_owned() name="params[password]".to_owned()
                            placeholder="Пароль".to_owned()
                            errors=move || ui_extract_field_errors("password", validation_errors)
                        />
                    </div>

                    <div class="field">
                        <div class="control">
                            <Button
                                class_name="is-primary".to_owned()
                                label="Войти".to_owned()
                                loading=login.pending()
                                on_click=move |_| {}
                                disabled=login.pending()
                            />
                        </div>
                    </div>
                </fieldset>
            </ActionForm>
        </div>
    }
}
