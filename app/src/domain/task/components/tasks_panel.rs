use leptos::prelude::*;

use crate::components::ui::button_link::ButtonLink;
use crate::components::ui::select_input::SelectInput;
use crate::domain::task::components::tasks_list::TasksList;
use crate::domain::task::model::task::sort_task;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::{get_filter_options, get_sort_options, get_tasks};
use crate::domain::user::model::user::User;

#[component]
pub fn TasksPanel() -> impl IntoView {
    let (filter, set_filter) = signal(Some("".to_owned()));

    let user = use_context::<ReadSignal<User>>().unwrap();

    let tasks_resource =
        Resource::new(move || user.get().id, move |_| async move { get_tasks(None, None).await });

    provide_context(tasks_resource);

    let filter_options_resource = OnceResource::new(get_filter_options());
    let sort_options_resource = OnceResource::new(get_sort_options());

    view! {
        <div class="container is-size-7-mobile pt-5">
            <div class="buttons is-justify-content-space-between px-2 pb-5">
                <Transition>
                    {move || Suspend::new(async move {
                        let filter_options = filter_options_resource.await.ok();
                        let sort_options = sort_options_resource.await.ok();
                        view! {
                                <span>
                                    <SelectInput
                                        class_name="is-size-7-mobile".to_owned()
                                        name="filter".to_owned()
                                        not_selected_text="Фильтр".to_owned()
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
                                        options=sort_options.unwrap()
                                        on_change=move |value: String| {
                                            let sort_kind = if value.is_empty() { None } else { Some(value) };
                                            tasks_resource.write().as_mut().map(|data|{
                                                if let Ok(tasks) = data {
                                                    tasks.sort_by(|task1, task2| sort_task(task1, task2, &sort_kind));
                                                }
                                            });
                                        }
                                    />
                                </span>

                                {if user.get().username.is_some() {
                                    view! {
                                        <ButtonLink
                                            class_name="level-item is-light is-size-7-mobile".to_owned()
                                            href=TaskRoutes::create_url().to_owned()
                                            label="+".to_owned()
                                            loading=None
                                        />
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}

                        }.into_any()
                    })}
                </Transition>

            </div>

            <TasksList filter />

        </div>
    }
}
