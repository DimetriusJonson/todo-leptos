use std::collections::HashMap;

use leptos::prelude::*;

use crate::{
    common::validate_helper::{ui_build_common_error, ui_build_validation_errors},
    components::{
        layout::message_banner::{show_info, Messages},
        ui::{button::Button, main_title::MainTitle, text_with_error::TextWithError},
    },
    domain::user::user_services::CreateUser,
};

#[component]
pub fn CreateUserPage() -> impl IntoView {
    let create_user = ServerAction::<CreateUser>::new();

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    let validation_errors: Signal<HashMap<String, Vec<String>>> =
        Signal::derive(move || create_user.value().with(ui_build_validation_errors));
    let common_error = move || ui_build_common_error(validation_errors);

    let api_in_progress = Signal::derive(move || create_user.pending().get());

    Effect::new(move |_| {
        if let Some(Ok(user)) = create_user.value().get() {
            show_info(
                format!("Создан пользователь {}", &user.username.unwrap()),
                messages,
            );
            create_user.clear();
        }
    });

    view! {
        <div class="container p-4">
            <MainTitle title="Создать пользователя".to_owned() />
            <ActionForm action=create_user>
                <div class="help is-danger is-size-5 py-4">{common_error}</div>

                <fieldset disabled=create_user.pending()>
                    <div class="field">
                        <TextWithError input_type="text".to_owned() name="params[name]".to_owned()
                            placeholder="Имя пользователя".to_owned()
                            errors=validation_errors
                            error_path="name"
                        />
                    </div>

                    <div class="field">
                        <TextWithError input_type="password".to_owned() name="params[password]".to_owned()
                            placeholder="Пароль".to_owned()
                            errors=validation_errors
                            error_path="password"
                        />
                    </div>

                    <div class="field">
                        <div class="control">
                            <Button
                                class_name="is-primary".to_owned()
                                label="Создать".to_owned()
                                loading=api_in_progress
                                on_click=move |_| {}
                                disabled=api_in_progress
                            />
                        </div>
                    </div>
                </fieldset>
            </ActionForm>
        </div>
    }
}
