use crate::SharedState;
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
        .close_kiosk_and_open_ha_kiosk()
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
pub async fn wake_up_display(Extension(state): Extension<SharedState>) {
    state.lock().await.hub_controller.wake_up_display().unwrap();
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
