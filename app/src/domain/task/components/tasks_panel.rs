use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::components::ui::button_link::ButtonLink;
use crate::components::ui::select_input::SelectInput;
use crate::domain::task::components::tasks_list::TasksList;
use crate::domain::task::model::task::{Task, sort_task};
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::{get_filter_options, get_sort_options, get_tasks};
use crate::domain::user::model::user::User;

#[component]
pub fn TasksPanel() -> impl IntoView {
    let (tasks, set_tasks) = signal(Vec::<Task>::new());
    let (filter, set_filter) = signal(Some("".to_owned()));

    let query_map = use_query_map();
    let filter_param = move || query_map.with(|m| m.get("filter"));
    let sort_kind_param = move || query_map.with(|m| m.get("sort_kind"));

    //let get_tasks_server_action = ServerAction::<GetTasks>::new();

    let user = use_context::<ReadSignal<User>>().unwrap();

    let tasks_resource = Resource::new(
        move || user.get().id,
        move |_| async move { get_tasks(filter_param(), sort_kind_param()).await },
    );
    let filter_options_resource = OnceResource::new(get_filter_options());
    let sort_options_resource = OnceResource::new(get_sort_options());

    view! {
            <div class="container is-size-7-mobile pt-5">
                <Suspense fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>
                    {move || Suspend::new(async move {
                        let filter_options = filter_options_resource.await.ok();
                        let sort_options = sort_options_resource.await.ok();
                        set_tasks.set(tasks_resource.await.ok().unwrap_or_default());

                        view! {
                            <div class="buttons is-justify-content-space-between px-2 pb-5">
                                <span>

                                   // <ActionForm action=get_tasks_server_action>
                                        <SelectInput
                                            class_name="is-size-7-mobile".to_owned()
                                            name="filter".to_owned()
                                            not_selected_text="Фильтр".to_owned()
                                            // value={filterSelect ?? tasksSettings.value.filter}
                                            options=filter_options.unwrap()
                                            on_change=move |value: String| {
                                                set_filter
                                                    .set(if value.is_empty() { None } else { Some(value.to_owned()) });
                                            }
                                        />
                                        <SelectInput
                                            class_name="is-size-7-mobile pl-2".to_owned()
                                            name="sort_kind".to_owned()
                                            not_selected_text="Сортировка".to_owned()
                                            // value={sortSelect ?? tasksSettings.value.sortKind}
                                            options=sort_options.unwrap()
                                            on_change=move |value: String| {
                                                let sort_kind = if value.is_empty() { None } else { Some(value) };
                                                set_tasks
                                                    .write()
                                                    .sort_by(|task1, task2| sort_task(task1, task2, &sort_kind));
                                            }
                                        />
    /*
                                        <Button
                                            class_name="level-item is-light is-size-7-mobile".to_owned()
                                            label="Ok".to_owned()
                                            on_click=move |_| {}
                                        />

                                    </ActionForm>
                                    */

                                </span>

                                {if user.get().username.is_some() {
                                    view! {
                                        <ButtonLink
                                            class_name="level-item is-light is-size-7-mobile".to_owned()
                                            href=TaskRoutes::create_url().to_owned()
                                            label="+".to_owned()
                                            loading=None
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}

                            </div>

                            // </form>

                            <TasksList tasks set_tasks filter />
                        }
                            .into_any()
                    })}
                </Suspense>
            </div>
        }
}
