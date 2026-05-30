use std::collections::HashMap;

use leptos::prelude::*;
use validator::Validate;
use web_sys::HtmlInputElement;

use crate::common::validate_helper::{
    extract_form_field_name, ui_build_common_error, ui_build_validation_errors, validate_field_value, validation_errors_to_map
};
use crate::components::layout::message_banner::{Messages, show_info};
use crate::components::ui::button::Button;
use crate::components::ui::main_title::MainTitle;
use crate::components::ui::text_with_error::TextWithError;
use crate::domain::user::model::create_user_params::CreateUserParams;
use crate::domain::user::user_services::CreateUser;

#[component]
pub fn CreateUserPage() -> impl IntoView {
    let create_user = ServerAction::<CreateUser>::new();

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    let (errors, set_validation_errors) = signal(HashMap::<String, Vec<String>>::new());

    let validation_errors: Signal<HashMap<String, Vec<String>>> = Signal::derive(move || {
        let mut result = errors.get();
        result.extend(create_user.value().with(ui_build_validation_errors));
        result
    });
    let common_error = move || ui_build_common_error(validation_errors);

    Effect::new(move |_| {
        if let Some(Ok(user)) = create_user.value().get() {
            show_info(format!("Создан пользователь {}", &user.username.unwrap()), messages);
            create_user.clear();
        }
    });

    view! {
        <div class="container p-4">
            <MainTitle title=|| "Создать пользователя".to_owned() />
            <ActionForm action=create_user 
                on:submit:capture=move |event| {
                    if let Ok(params) = CreateUser::from_event(&event) {
                        if let Err(validation_errors) = params.validate() {
                            set_validation_errors.set(validation_errors_to_map(validation_errors));
                            event.prevent_default();
                        }
                    } else {
                        event.prevent_default();
                    }
                }
                on:input=move |event| {
                        let target = event_target::<HtmlInputElement>(&event);
                        let field_name = extract_form_field_name(target.name().to_owned());
                        set_validation_errors.write().insert(field_name.to_owned(), validate_field_value(field_name.to_owned(), target.value(), CreateUserParams::default()));
                        create_user.clear();                    
                    }
            >
                <input name="params[version]" type="hidden" value={move || create_user.version().get()} />

                <div class="help is-danger is-size-5 py-4">{common_error}</div>

                <fieldset disabled=create_user.pending()>
                    <div class="field">
                        <TextWithError
                            input_type="text".to_owned()
                            name="params[name]".to_owned()
                            placeholder="Имя пользователя".to_owned()
                            validation_errors
                        />
                    </div>

                    <div class="field">
                        <TextWithError input_type="password".to_owned() name="params[password]".to_owned()
                            placeholder="Пароль".to_owned()
                            validation_errors
                        />
                    </div>

                    <div class="field">
                        <div class="control">
                            <Button
                                class_name="is-primary".to_owned()
                                label="Создать".to_owned()
                                loading=move || create_user.pending().get()
                                on_click=move |_| {}
                                disabled=move || create_user.pending().get()
                            />
                        </div>
                    </div>
                </fieldset>
            </ActionForm>
        </div>
    }
}
