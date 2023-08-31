use std::{
    net::SocketAddr,
    process::Command,
    sync::{Arc, Mutex},
};

use crate::config::AppConfig;
use axum::Router;
use cli::parse_args;
use tokio::sync::broadcast;

pub mod cli;
pub mod config;
pub mod error;
pub mod models;
pub mod prelude;
pub mod routes;
pub mod services;

#[tokio::main]
async fn main() {
    let m = parse_args();
    let cfg = AppConfig::new(&m).unwrap();

    let (tx, _) = broadcast::channel::<String>(1);

    let app_state = AppState {
        tx: tx.clone(),
        snapshot: Arc::new(Mutex::new("".to_string())),
    };

    let addr: SocketAddr = format!("{}:{}", &cfg.server_host, &cfg.server_port)
        .parse()
        .expect("failed to parse address and port");

    let app = Router::new()
        .merge(services::fron_public_route())
        .merge(services::back_public_route(app_state.clone()));

    // Update current time in the background
    tokio::task::spawn_blocking(move || {
        loop {
            let cmd_output = Command::new("date").output().unwrap().stdout;
            let cmd_output_fmt = std::str::from_utf8(&cmd_output).unwrap();
            let _ = tx.send(cmd_output_fmt.to_string());
            // let mut cur_date = app_state.snapshot.lock().unwrap();
            // *cur_date = cmd_output_fmt.to_string();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    tracing::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    tx: broadcast::Sender<String>,
    pub snapshot: Arc<Mutex<String>>,
}
