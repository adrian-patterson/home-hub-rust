mod routes;
mod utils;

use crate::{
    routes::homehub_router::{close_chrome_kiosk, wake_up_display, sleep_display},
    utils::hubcontroller::HubController,
};
use anyhow::Error;
use axum::{routing::post, Extension, Router};
use routes::homehub_router::{self, open_chrome_kiosk};
use tokio_cron_scheduler::{JobScheduler, Job};
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
    // Initialize schedulers to turn screen on/off on a schedule
    initialize_schedulers().await.unwrap();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    #[derive(OpenApi)]
    #[openapi(
        paths(
            homehub_router::open_chrome_kiosk, 
            homehub_router::close_chrome_kiosk, 
            homehub_router::wake_up_display,
            homehub_router::sleep_display,
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
        .route("/homehub/wake", post(wake_up_display))
        .route("/homehub/sleep", post(sleep_display))
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
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
}

async fn initialize_schedulers() -> Result<(), Error> {
    let sched = JobScheduler::new().await?;
  
    sched.add(
        Job::new_async("0 0 12 * * *", |_uuid, _l| {
            Box::pin(async move {
                let mut hub_controller = HubController::new();
                hub_controller.wake_up_display().unwrap();
            })
        })?
    ).await?;

    sched.add(
        Job::new_async("0 0 2 * * *", |_uuid, _l| {
            Box::pin(async move {
                let mut hub_controller = HubController::new();
                hub_controller.close_kiosk_and_open_ha_kiosk().unwrap();
                hub_controller.sleep_display().unwrap();
            })
        })?
    ).await?;

    sched.start().await?;

    Ok(())
}