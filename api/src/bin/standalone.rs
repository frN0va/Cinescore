//! The standalone cinescore API binary
use std::net::Ipv4Addr;

use api::build_router;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        panic!(".env file does not exist");
    }

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cinescore_api=info,api=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let variables = ["TMDB_API_KEY"];

    for var in variables {
        assert!(
            std::env::var(var).is_ok(),
            "environment variable {} is not set",
            var
        );
    }

    // connect to DB
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    tracing::info!("Connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    tracing::info!("Running database migrations");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let args = Cli::parse();
    let address = format!("{}:{}", args.bind, args.port);

    let router = build_router(pool).await;

    let listener = match tokio::net::TcpListener::bind(&address).await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Error while binding to address {}: {}", address, e);
            std::process::exit(1);
        }
    };

    tracing::info!("Bound to http://{}", address);

    match axum::serve(listener, router).await {
        Ok(_) => (),
        Err(e) => tracing::error!("Error serving axum app: {}", e),
    };

    Ok(())
}
