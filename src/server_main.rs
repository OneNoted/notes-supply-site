#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,notes_supply_site=debug,tower_http=debug".into()),
        )
        .init();

    notes_supply_site::server::run().await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("notes-supply-site-server is native-only");
}
