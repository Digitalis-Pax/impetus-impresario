use tracing::{info, instrument};

#[instrument]
pub async fn run() {
    info!("Core initialized");
}

#[cfg(test)]
mod tests {}
