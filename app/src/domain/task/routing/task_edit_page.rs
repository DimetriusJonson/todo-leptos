use std::collections::HashMap;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::common::validate_helper::{ui_build_common_error, ui_build_validation_errors};
use crate::components::layout::message_banner::{Messages, show_info};
use crate::components::ui::button::Button;
use crate::components::ui::button_link::ButtonLink;
use crate::components::ui::checkbox_with_label::CheckboxWithLabel;
use crate::components::ui::main_title::MainTitle;
use crate::components::ui::select_with_label::SelectWithLabel;
use crate::components::ui::text_area::TextArea;
use crate::components::ui::text_with_error::TextWithError;
use crate::domain::home::routing::routes::HomeRoutes;
use crate::domain::task::model::task::Task;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::{UpdateOrCreateTask, get_priorities, get_task};

#[component]
pub fn TaskEditPage() -> impl IntoView {
    let update_or_create_task = ServerAction::<UpdateOrCreateTask>::new();

    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default().parse::<i64>().ok();

    let messages = use_context::<Messages>().expect("Cant get messages context!");
    let task_resource = Resource::new(id, move |id| get_task(id.unwrap_or(0)));
    let priorities_resource = OnceResource::new(get_priorities());

    let validation_errors: Signal<HashMap<String, Vec<String>>> =
        Signal::derive(move || update_or_create_task.value().with(ui_build_validation_errors));
    let common_error = move || ui_build_common_error(validation_errors);

    let api_in_progress = Signal::derive(move || update_or_create_task.pending().get());

    Effect::new(move |_| {
        if let Some(Ok(_)) = update_or_create_task.value().get() {
            show_info("Задача сохранена!".to_owned(), messages);
            update_or_create_task.clear();
        }
    });

    let title = move || match id() {
        Some(_) => "Редактировать задачу".to_owned(),
        None => "Создать задачу".to_owned(),
    };

    view! {
        <div class="container p-4">
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || Suspend::new(async move {
                    let task = if id().is_some() { task_resource.await.unwrap() } else { Task::default() };
                    let priorities = priorities_resource.await.ok();
                    view! {
                        <MainTitle title=title() />
                        <ActionForm action=update_or_create_task>
                            <input type="hidden" name="task[id]" value=task.id />

                            <div class="help is-danger is-size-5 py-4">{common_error}</div>

                            <fieldset disabled=update_or_create_task.pending()>
                                <div class="level">
                                    <div class="level-left">
                                        <div class="level-item">
                                            <SelectWithLabel
                                                name="task[priority]".to_owned()
                                                label="Приоритет:".to_owned()
                                                error_path="priority"
                                                errors=validation_errors
                                                options=priorities.unwrap()
                                                not_selected_text="Не выбран".to_owned()
                                                value=task.priority.unwrap_or_default()
                                                on_change=|_| {}
                                            />
                                        </div>
                                    </div>

                                    <div class="level-right">
                                        <div class="level-item">
                                            <CheckboxWithLabel
                                                name="task[completed_at]".to_owned()
                                                value=task.completed_at.is_some()
                                                label="Завершена".to_owned()
                                            />
                                        </div>
                                    </div>

                                </div>
                                <div class="field">
                                    <TextWithError
                                        input_type="text".to_owned()
                                        name="task[title]".to_owned()
                                        placeholder="Название".to_owned()
                                        errors=validation_errors
                                        error_path="title"
                                        value=task.title.unwrap_or_default()
                                    />
                                </div>

                                <div class="field">
                                    <TextArea
                                        name="task[description]".to_owned()
                                        placeholder="Описание".to_owned()
                                        value=task.description.unwrap_or_default()
                                        on_change=|_| {}
                                    />
                                </div>

                                <div class="field is-grouped">
                                    <div class="control">

                                        <Button
                                            class_name="is-primary".to_owned()
                                            label="Сохранить".to_owned()
                                            loading=api_in_progress
                                            on_click=move |_| {}
                                        />
                                    </div>
                                    <div class="control">
                                        <ButtonLink
                                            class_name="is-light".to_owned()
                                            label="Отмена".to_owned()
                                            href=match task.id {
                                                Some(id) => TaskRoutes::details_url(id),
                                                None => HomeRoutes::base_url().to_owned(),
                                            }
                                                .to_owned()
                                            loading=None
                                        />
                                    </div>
                                </div>

                            </fieldset>
                        </ActionForm>
                    }
                })}
            </Suspense>
        </div>
    }
}
