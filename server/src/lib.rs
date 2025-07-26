mod configuration;
mod logging;

use crate::configuration::Configuration;
use log::{debug, error, info, trace, warn};

pub async fn run() {
    let configuration = Configuration::load();
    configuration.logging.init();
    println!("Configuration:\n{configuration:?}");
    trace!("Trace enabled");
    debug!("Debug enabled");
    info!("Info enabled");
    warn!("Warn enabled");
    error!("Error enabled");
}
