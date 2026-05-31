use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, Response, StatusCode, Uri};
use axum::response::{IntoResponse, Response as AxumResponse};
use http::HeaderValue;
use http::header::CACHE_CONTROL;
use leptos::prelude::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use app::common::errors::{AppError, ErrorBoundary};

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let mut res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        let cache_control_value = HeaderValue::from_static("public, immutable, max-age=31536000");
        res.headers_mut().insert(CACHE_CONTROL, cache_control_value);
        res.into_response()
    } else {
        let mut errors = Errors::default();
        errors.insert_with_default_key(AppError::NotFound(uri.to_string()));
        let handler = leptos_axum::render_app_to_stream(
            move || view! { <ErrorBoundary outside_errors=errors.clone() /> },
        );
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {err}")))
        }
    }
}
