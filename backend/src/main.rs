use args::ProgramArgs;
use axum::{Router, routing::get};
use clap::Parser;
use handlers::get_index;
use tokio::net;
use tower_http::{
    LatencyUnit,
    services::ServeDir,
    trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{Level, event, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt as _};

mod args;
mod handlers;
mod states;

/// The main function for he backend
#[tokio::main]
async fn main() {
    // Parse the program arguments
    let program_args = ProgramArgs::parse();

    // Start logging
    let tracing_filter = tracing_subscriber::filter::LevelFilter::from_level(Level::INFO);
    let (tracing_layer, reload_handle) = tracing_subscriber::reload::Layer::new(tracing_filter);
    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Log that we're starting
    event!(Level::INFO, "Starting ConnectIA backend!");

    // Parse the verbosity from the command line arguments
    // Otherwise default to info
    let verbosity = match program_args.verbosity {
        Some(level) => {
            let level = match level.parse::<Level>() {
                Ok(level) => level,
                Err(err) => {
                    event!(Level::ERROR, "Failed to parse verbosity level: {}", err);
                    Level::INFO
                }
            };
            event!(Level::INFO, "Setting verbosity level to {}", level);
            level
        }
        None => {
            event!(
                Level::INFO,
                "No verbosity level provided, defaulting to INFO"
            );
            Level::INFO
        }
    };

    // Change the verbosity of the logger
    match reload_handle.modify(|filter| *filter = LevelFilter::from_level(verbosity)) {
        Ok(_) => event!(Level::INFO, "Verbosity level changed to {}", verbosity),
        Err(err) => event!(Level::ERROR, "Failed to change verbosity level: {}", err),
    }

    // Get the port from the command line arguments
    let port = match program_args.port {
        Some(port) => {
            event!(Level::INFO, "Setting port to {}", port);
            port
        }
        None => {
            event!(Level::INFO, "No port provided, defaulting to 8080");
            8080
        }
    };

    // Get the static directory from the command line arguments
    let static_dir = match program_args.static_dir {
        Some(dir) => {
            event!(Level::INFO, "Setting static directory to {}", dir.display());
            dir
        }
        None => {
            event!(
                Level::INFO,
                "No static directory provided, defaulting to ../static"
            );
            "../static".parse().unwrap()
        }
    };

    // Create the root state
    let root_state = states::RootState::new(&static_dir);

    // Create the root router
    let root_router = Router::new()
        .route("/", get(handlers::get_index))
        .fallback(get_index)
        .nest_service("/static", ServeDir::new(&static_dir))
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::DEBUG)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .with_state(root_state);

    // Create the listener
    let listener = match net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(listener) => {
            event!(Level::INFO, "Listener created on port {}", port);
            listener
        }
        Err(err) => {
            event!(Level::ERROR, "Failed to create listener: {}", err);
            panic!("Failed to create listener: {}", err);
        }
    };

    // Serve the root router
    match axum::serve(listener, root_router).await {
        Ok(_) => event!(Level::INFO, "Finished serving on port {}", port),
        Err(err) => event!(Level::ERROR, "Failed to serve: {}", err),
    };
}
