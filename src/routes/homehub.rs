use crate::{utils::hubcontroller::HubController, SharedState};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
#[schema(example = json!({ "url": "https://google.com" }))]
pub struct OpenUrl {
    url: String,
}

#[axum_macros::debug_handler]
#[utoipa::path(
        post,
        path = "/homehub/kiosk",
        tag = "Homehub",
        request_body = OpenUrl,
        responses(
            (status = 200, description = "Chrome instance in Kiosk mode was opened with given URL"),
        ),
    )]
pub async fn open_chrome_kiosk(
    Extension(state): Extension<SharedState>,
    Json(open_url): Json<OpenUrl>,
) {
    state
        .lock()
        .await
        .hub_controller
        .open_chrome_kiosk(open_url.url)
        .expect("Error opening chrome");
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
        .expect("Error closing chrome");
}

#[axum_macros::debug_handler]
#[utoipa::path(
        post,
        path = "/homehub/browser",
        tag = "Homehub",
        request_body = OpenUrl,
        responses(
            (status = 200, description = "Firefox instance opened in windowed fullscreen."),
        ),
    )]
pub async fn open_firefox(Extension(state): Extension<SharedState>, Json(open_url): Json<OpenUrl>) {
    state
        .lock()
        .await
        .hub_controller
        .open_firefox(open_url.url)
        .expect("Error opening firefox");
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
    HubController::wake_up_display().expect("Error waking display");
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
pub async fn sleep_display() {
    HubController::sleep_display().expect("Error waking display");
}
