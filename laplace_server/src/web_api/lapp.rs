use axum::routing::{any, get, post};
use axum::Router;
use const_format::concatcp;

use crate::lapps::{Lapp, LappsProvider};

pub mod handler;

pub fn router() -> Router<LappsProvider> {
    Router::new()
        .route("/:lapp_name", get(handler::index_file))
        .route(
            concatcp!("/:lapp_name/", Lapp::static_dir_name(), "/*file_path"),
            get(handler::static_file),
        )
        .route("/:lapp_name/api/ws", get(handler::ws_start))
        .route("/:lapp_name/api/p2p", post(handler::gossipsub_start))
        .route("/:lapp_name/api/*tail", any(handler::http))
        .route("/:lapp_name/*tail", get(handler::index))
}
