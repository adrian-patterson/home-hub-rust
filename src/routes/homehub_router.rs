use crate::{utils::hubcontroller::HubController, SharedState};
use axum::{extract::Query, Extension};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct UrlQuery {
    url: String,
}

#[axum_macros::debug_handler]
#[utoipa::path(
        post,
        path = "/homehub/kiosk",
        tag = "Homehub",
        params(
            UrlQuery
        ),
        responses(
            (status = 200, description = "Chrome instance in Kiosk mode was opened with given URL"),
        ),
    )]
pub async fn open_chrome_kiosk(
    Extension(state): Extension<SharedState>,
    Query(url_query): Query<UrlQuery>,
) {
    state
        .lock()
        .await
        .hub_controller
        .open_chrome_kiosk(url_query.url)
        .unwrap();
}

#[axum_macros::debug_handler]
#[utoipa::path(
        delete,
        path = "/homehub/kiosk",
        tag = "Homehub",
        responses(
            (status = 200, description = "Chrome Kiosk closed"),
        ),
    )]
pub async fn close_chrome_kiosk(Extension(state): Extension<SharedState>) {
    state
        .lock()
        .await
        .hub_controller
        .close_chrome_kiosk()
        .unwrap();
}

#[axum_macros::debug_handler]
#[utoipa::path(
        post,
        path = "/homehub/browser",
        tag = "Homehub",
        params(
            UrlQuery
        ),
        responses(
            (status = 200, description = "Firefox instance opened in windowed fullscreen."),
        ),
    )]
pub async fn open_firefox(
    Extension(state): Extension<SharedState>,
    Query(url_query): Query<UrlQuery>,
) {
    state
        .lock()
        .await
        .hub_controller
        .open_firefox(url_query.url)
        .unwrap();
}

#[axum_macros::debug_handler]
#[utoipa::path(
        post,
        path = "/homehub/wake",
        tag = "Homehub",
        responses(
            (status = 200, description = "Display woken up."),
        ),
    )]
pub async fn wake_up_display() {
    HubController::wake_up_display().unwrap();
}

#[axum_macros::debug_handler]
#[utoipa::path(
        post,
        path = "/homehub/sleep",
        tag = "Homehub",
        responses(
            (status = 200, description = "Display woken up."),
        ),
    )]
pub async fn sleep_display(Extension(state): Extension<SharedState>) {
    state.lock().await.hub_controller.sleep_display().unwrap();
}
