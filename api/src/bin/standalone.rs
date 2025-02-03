//! The standalone cinescore API binary
use api::build_router;

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

    let router = build_router();

    let address = if cfg!(debug_assertions) {
        "127.0.0.1:8000"
    } else {
        "0.0.0.0:8000"
    };

    let listener = match tokio::net::TcpListener::bind(address).await {
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
