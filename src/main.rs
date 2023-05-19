mod routes;
mod utils;

use crate::{
    routes::homehub::{close_chrome_kiosk, open_firefox, wake_up_display, sleep_display},
    utils::hubcontroller::HubController,
};
use axum::{routing::post, Extension, Router};
use routes::homehub::{self, open_chrome_kiosk};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

type SharedState = Arc<Mutex<State>>;

pub struct State {
    hub_controller: HubController,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    #[derive(OpenApi)]
    #[openapi(
        paths(
            homehub::open_chrome_kiosk, 
            homehub::close_chrome_kiosk, 
            homehub::open_firefox, 
            homehub::wake_up_display,
            homehub::sleep_display
        ),
        components(
            schemas(homehub::OpenUrl)
        ),
        tags(
            (name = "Homehub", description = "Homehub controller API")
        )
    )]
    struct ApiDoc;

    let shared_state = Arc::new(Mutex::new(State {
        hub_controller: HubController::new(),
    }));

    // Create router with logging
    let router = Router::new()
        .route(
            "/homehub/kiosk",
            post(open_chrome_kiosk).delete(close_chrome_kiosk),
        )
        .route("/homehub/browser", post(open_firefox))
        .route("/homehub/wake", post(wake_up_display))
        .route("/homehub/sleep", post(sleep_display))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(Extension(shared_state));

    info!("Server running at http://localhost:5000/docs");
    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();

    HubController::prevent_screen_sleep().unwrap();
}
