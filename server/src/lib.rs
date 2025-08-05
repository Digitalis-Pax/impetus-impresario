mod api;
mod configuration;
mod logging;

use crate::{api::Api, configuration::Configuration};
use log::info;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub async fn run() {
    let app_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState { ready: false }));

    let configuration: Configuration = Configuration::load();
    configuration.logging.init();

    let mut api = Api::new(&configuration.api);
    api.serve(&app_state).await;

    let mon_state = app_state.clone();
    let monitor_handle = tokio::spawn(async move {
        monitor(mon_state).await;
    });

    main_loop();

    api.shutdown();

    monitor_handle.await.unwrap();
}

async fn monitor(app_state: Arc<Mutex<AppState>>) {
    info!("Monitoring");
    thread::sleep(Duration::from_secs(10));
    app_state.lock().unwrap().ready = true;
}

fn main_loop() {
    info!("Started successfully. Press `Ctrl-C` to shutdown.");

    let shutdown_requested = Arc::new(Mutex::new(false));

    let ctrlc_shutdown_requested = shutdown_requested.clone();

    ctrlc::set_handler(move || {
        info!("Shutdown requested from the terminal");
        *ctrlc_shutdown_requested.lock().unwrap() = true;
    })
    .unwrap();

    while !*shutdown_requested.lock().unwrap() {
        thread::sleep(Duration::from_millis(100));
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AppState {
    pub ready: bool,
}
