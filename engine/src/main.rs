use std::error::Error;

use impetus_impresario::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await
}
