use std::collections::HashMap;

use leptos::prelude::*;

use crate::common::validate_helper::{
    ui_build_common_error, ui_build_validation_errors, ui_extract_field_errors,
};
use crate::components::layout::message_banner::{Messages, show_info};
use crate::components::ui::button::Button;
use crate::components::ui::main_title::MainTitle;
use crate::components::ui::text_with_error::TextWithError;
use crate::domain::user::user_services::CreateUser;

#[component]
pub fn CreateUserPage() -> impl IntoView {
    let create_user = ServerAction::<CreateUser>::new();

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    let validation_errors: Signal<HashMap<String, Vec<String>>> =
        Signal::derive(move || create_user.value().with(ui_build_validation_errors));
    let common_error = move || ui_build_common_error(validation_errors);

    Effect::new(move |_| {
        if let Some(Ok(user)) = create_user.value().get() {
            show_info(format!("Создан пользователь {}", &user.username.unwrap()), messages);
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
                            errors=move || ui_extract_field_errors("name", validation_errors)
                        />
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
                                label="Создать".to_owned()
                                loading=create_user.pending()
                                on_click=move |_| {}
                                disabled=create_user.pending()
                            />
                        </div>
                    </div>
                </fieldset>
            </ActionForm>
        </div>
    }
}
