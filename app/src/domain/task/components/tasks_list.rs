use leptos::prelude::*;
use leptos::reactive::spawn_local;
use web_sys::{Event, HtmlInputElement};

use crate::components::layout::message_banner::{Messages, show_error, show_info};
use crate::components::ui::checkbox::Checkbox;
use crate::domain::task::model::task::{Task, filter_task};
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::change_completed_task;

#[component]
pub fn TasksList(
    tasks: ReadSignal<Vec<Task>>,
    set_tasks: WriteSignal<Vec<Task>>,
    filter: ReadSignal<Option<String>>,
) -> impl IntoView {
    let messages: Messages = use_context::<Messages>().expect("Cant get messages context!");

    let completed_on_change = move |event: Event| {
        event.prevent_default();

        let checkbox = event_target::<HtmlInputElement>(&event);
        let name = checkbox.name();
        let value = checkbox.checked();
        checkbox.set_checked(!value);

        if let Some(index_und) = name.find('_') {
            if let Ok(id) = name[index_und + 1..].parse::<i64>() {
                spawn_local(async move {
                    checkbox.set_disabled(true);
                    let res = change_completed_task(id, value).await;
                    checkbox.set_disabled(false);
                    match res {
                        Ok(saved_task) => {
                            checkbox.set_checked(saved_task.completed_at.is_some());

                            if let Some(found_task) =
                                set_tasks.write().iter_mut().find(|t| t.id == Some(id))
                            {
                                found_task.completed_at = saved_task.completed_at;
                            }
                            show_info("Задача сохранена.".to_owned(), messages);
                        }
                        Err(err) => {
                            let msg = match err {
                                ServerFnError::ServerError(err) => err,
                                _ => err.to_string(),
                            };
                            show_error(msg, messages);
                        }
                    }
                });
            }
        }
    };

    view! {
        <table class="table is-striped is-fullwidth">
            <thead>
                <tr>
                    <th>{"Приоритет"}</th>
                    <th>{"Завершена"}</th>
                    <th>{"Название"}</th>
                    <th class="is-hidden-mobile">{"Описание"}</th>
                </tr>
            </thead>
            <tbody>
                {move || {
                    if !tasks.read().is_empty() {
                        {
                            tasks
                                .get()
                                .into_iter()
                                .filter(|task| filter_task(task, &filter.get()))
                                .map(|task| {
                                    view! {
                                        <tr>
                                            <td>{task.priority_name()}</td>
                                            <td>
                                                <Checkbox
                                                    class_name="is-medium".to_owned()
                                                    name=format!("completed_{}", task.id.unwrap())
                                                    value=task.completed_at.is_some()
                                                    title=match &task.completed_at {
                                                        Some(completed_at) => completed_at.to_owned(),
                                                        None => "".to_owned(),
                                                    }
                                                    on:change=completed_on_change
                                                />
                                            </td>
                                            <td>
                                                <a
                                                    href=TaskRoutes::details_url(task.id.unwrap())
                                                    aria-label=task.title.to_owned()
                                                >
                                                    {task.title.to_owned()}
                                                </a>
                                            </td>
                                            <td class="is-hidden-mobile">{task.description.to_owned()}</td>
                                        </tr>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }
                            .into_any()
                    } else {

                        view! {
                            <tr>
                                <td colSpan="3" style="text-align: center">
                                    Нет записей
                                </td>
                            </tr>
                        }
                            .into_any()
                    }
                }}
            </tbody>
        </table>
    }
}

