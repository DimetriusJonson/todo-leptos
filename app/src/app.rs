use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes};
use leptos_router::{StaticSegment, path};

use crate::components::layout::message_banner::MessageBanner;
use crate::components::layout::navbar::Navbar;
use crate::domain::home::routing::home_page::HomePage;
use crate::domain::home::routing::routes::HomeRoutes;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::routing::task_edit_page::TaskEditPage;
use crate::domain::task::routing::task_page::TaskPage;
use crate::domain::user::routing::create_user_page::CreateUserPage;
use crate::domain::user::routing::login_page::LoginPage;
use crate::domain::user::routing::routes::UserRoutes;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ru">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="text-scale" content="scale" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="bulma" href="https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css" />
        <Stylesheet id="leptos" href="/pkg/todo_leptos.css" />

        <section class="section p-0">
            <div class="is-paddingless">
                <Router>
                    <main>
                        <MessageBanner />
                        <Navbar />
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=StaticSegment(HomeRoutes::base_segment()) view=HomePage />

                            <ParentRoute path=StaticSegment(UserRoutes::base_segment()) view=Outlet>
                                <Route path=StaticSegment(UserRoutes::create_segment()) view=CreateUserPage />
                                <Route path=StaticSegment(UserRoutes::login_segment()) view=LoginPage />
                            </ParentRoute>

                            <ParentRoute path=StaticSegment(TaskRoutes::base_segment()) view=Outlet>
                                <Route path=StaticSegment(TaskRoutes::create_segment()) view=TaskEditPage />
                                <Route path=path!(":id") view=TaskPage />
                                <Route path=path!(":id/edit") view=TaskEditPage />
                            </ParentRoute>

                        </Routes>
                    </main>
                </Router>
            </div>
        </section>
    }
}
