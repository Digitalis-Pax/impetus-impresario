use tracing::{info, instrument};

#[instrument]
#[tokio::main]
async fn main() {
    init_tracing();
    impetus_impresario_core::run().await;
}

#[instrument]
fn init_tracing() {
    let subscriber = tracing_subscriber::fmt().pretty().finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"))
        .unwrap();
    info!("Tracing initialized");
}
