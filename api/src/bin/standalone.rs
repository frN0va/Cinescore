//! The standalone cinescore API binary
use std::net::Ipv4Addr;

use api::build_router;
use clap::{arg, command, Parser};

/// CLI Arguments struct
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specify an address to bind the server to
    #[arg(short, long, default_value_t = Ipv4Addr::LOCALHOST)]
    bind: Ipv4Addr,
    /// Specify a port to run the server on
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load dotenv file if exists, and if not, check that environment variable is defined
    if dotenvy::from_filename(".env").is_err() {
        assert!(
            std::env::var("TMDB_API_KEY").is_ok(),
            ".env file does not exists and environment variable TMDB_API_KEY is not set"
        )
    }

    assert!(
        std::env::var("TMDB_API_KEY").is_ok(),
        "environment variable TMDB_API_KEY is not set"
    );

    let args = Cli::parse();
    let address = format!("{}:{}", args.bind, args.port);

    let router = build_router();

    let listener = match tokio::net::TcpListener::bind(&address).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error while binding to address {}: {}", address, e);
            std::process::exit(1);
        }
    };

    log::info!("Bound to http://{}", address);

    match axum::serve(listener, router).await {
        Ok(_) => (),
        Err(e) => log::error!("Error serving axum app: {}", e),
    };

    Ok(())
}
