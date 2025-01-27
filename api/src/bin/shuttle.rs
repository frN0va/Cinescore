use api::build_router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = build_router();

    Ok(router.into())
}
