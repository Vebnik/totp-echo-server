mod types;
mod api;
mod impls;
mod error;
mod router;
mod handlers;

use clap::Parser;

use error::Result;
use types::Args;
use router::serve;


#[tokio::main]
async fn main() -> Result<()> {
    let cmd_args = Args::parse();

    dotenv::dotenv().expect("Error on read .env");
    env_logger::init();

    serve(cmd_args).await;

    Ok(())
}
