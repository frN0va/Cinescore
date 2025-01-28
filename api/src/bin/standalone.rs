use api::build_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename("Secrets.toml")?;

    let router = build_router();

    let address = "127.0.0.1:8000";

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Bound to http://{}", address);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
