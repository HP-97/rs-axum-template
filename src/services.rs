use axum::{handler::HandlerWithoutStateExt, http::StatusCode, routing::get, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{routes, AppState};

pub fn fron_public_route() -> Router {
    Router::new()
        .fallback_service(ServeDir::new("./web-src").not_found_service(handle_error.into_service()))
        .layer(TraceLayer::new_for_http())
}

#[allow(clippy::unused_async)]
async fn handle_error() -> (StatusCode, &'static str) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong access static files...",
    )
}

pub fn back_public_route(app_state: AppState) -> Router {
    Router::new()
        .route("/example", get(routes::api::example()))
        .route("/get_distro", get(routes::api::get_distro()))
        .route("/get_datetime", get(routes::api::get_date))
        .route(
            "/get_datetime_realtime",
            get(routes::api::get_datetime_realtime),
        )
        .route(
            "/get_datetime_realtime_sse",
            get(routes::api::get_datetime_realtime_sse),
        )
        .with_state(app_state.clone())
}
