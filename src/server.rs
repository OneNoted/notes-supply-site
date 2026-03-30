#[cfg(not(target_arch = "wasm32"))]
use crate::types::{GithubStats, LangStat, ProjectListItem};

#[cfg(not(target_arch = "wasm32"))]
use axum::{
    Json, Router,
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::get,
};

#[cfg(not(target_arch = "wasm32"))]
use tower_http::services::{ServeDir, ServeFile};

#[cfg(not(target_arch = "wasm32"))]
pub async fn run() {
    crate::github::warm_cache().await;
    let static_root = static_root();
    let state = AppState { static_root };
    let bind_addr = bind_addr();

    let api = Router::new()
        .route("/stats", get(api_stats))
        .route("/projects", get(api_projects))
        .route("/project/{slug}", get(api_project))
        .route("/readme/{slug}", get(api_readme));

    let app = Router::new()
        .route("/healthz", get(healthz))
        .nest("/api", api)
        .nest_service("/assets", ServeDir::new(format!("{}/assets", state.static_root)))
        .nest_service("/app", ServeDir::new(format!("{}/app", state.static_root)))
        .nest_service("/fonts", ServeDir::new(format!("{}/fonts", state.static_root)))
        .nest_service("/wasm", ServeDir::new(format!("{}/wasm", state.static_root)))
        .route_service(
            "/apple-touch-icon.png",
            ServeFile::new(format!("{}/apple-touch-icon.png", state.static_root)),
        )
        .route_service(
            "/favicon.ico",
            ServeFile::new(format!("{}/favicon.ico", state.static_root)),
        )
        .route_service(
            "/favicon.svg",
            ServeFile::new(format!("{}/favicon.svg", state.static_root)),
        )
        .fallback(get(spa_index))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind server");

    tracing::info!("notes-supply-site server listening on http://{bind_addr}");
    tracing::info!("serving static files from {}", state.static_root);
    axum::serve(listener, app).await.expect("server exited unexpectedly");
}

#[cfg(not(target_arch = "wasm32"))]
async fn healthz() -> &'static str {
    "ok"
}

#[cfg(not(target_arch = "wasm32"))]
async fn api_stats() -> Json<GithubStats> {
    let projects = crate::github::get_projects().await;
    let repos = projects.len();
    let stars = projects.iter().map(|project| project.stars).sum();
    let forks = projects.iter().map(|project| project.forks).sum();
    let latest_project = projects
        .first()
        .map(|project| project.name.clone())
        .unwrap_or_default();
    let latest_updated = projects
        .first()
        .map(|project| relative_time(&project.updated_at))
        .unwrap_or_else(|| "unknown".into());
    let languages = crate::github::get_languages()
        .await
        .into_iter()
        .map(|(name, percent)| LangStat { name, percent })
        .collect();

    Json(GithubStats {
        repos,
        stars,
        forks,
        contributions: crate::github::get_contributions().await,
        languages,
        latest_project,
        latest_updated,
    })
}

#[cfg(not(target_arch = "wasm32"))]
async fn api_projects() -> Json<Vec<ProjectListItem>> {
    let projects = crate::github::get_projects().await;
    Json(projects
        .iter()
        .map(|project| ProjectListItem {
            name: project.name.clone(),
            slug: project.slug.clone(),
            language: project.language.clone().unwrap_or_else(|| "—".into()),
            stars: project.stars,
            updated: relative_time(&project.updated_at),
        })
        .collect())
}

#[cfg(not(target_arch = "wasm32"))]
async fn api_project(Path(slug): Path<String>) -> Result<Json<crate::types::Project>, axum::http::StatusCode> {
    crate::github::get_project(&slug)
        .await
        .map(Json)
        .ok_or(axum::http::StatusCode::NOT_FOUND)
}

#[cfg(not(target_arch = "wasm32"))]
async fn api_readme(Path(slug): Path<String>) -> Result<String, axum::http::StatusCode> {
    crate::github::fetch_readme(&slug)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)
}

#[cfg(not(target_arch = "wasm32"))]
fn relative_time(iso: &str) -> String {
    let now = chrono::Utc::now();
    if let Ok(date_time) = chrono::DateTime::parse_from_rfc3339(iso) {
        let delta = now.signed_duration_since(date_time);
        if delta.num_days() > 365 {
            format!("{}y ago", delta.num_days() / 365)
        } else if delta.num_days() > 30 {
            format!("{}mo ago", delta.num_days() / 30)
        } else if delta.num_days() > 0 {
            format!("{}d ago", delta.num_days())
        } else if delta.num_hours() > 0 {
            format!("{}h ago", delta.num_hours())
        } else {
            "just now".into()
        }
    } else {
        "unknown".into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn bind_addr() -> String {
    std::env::var("NOTES_SUPPLY_SITE_BIND_ADDR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "127.0.0.1:4000".into())
}

#[cfg(not(target_arch = "wasm32"))]
fn static_root() -> String {
    if let Ok(root) = std::env::var("NOTES_SUPPLY_SITE_WEB_ROOT") {
        return root;
    }

    let candidates = [
        "target/dx/notes-supply-site/release/web/public",
        "target/dx/notes-supply-site/debug/web/public",
        "dist",
    ];

    candidates
        .into_iter()
        .find(|path| std::path::Path::new(path).exists())
        .unwrap_or("target/dx/notes-supply-site/debug/web/public")
        .to_string()
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
struct AppState {
    static_root: String,
}

#[cfg(not(target_arch = "wasm32"))]
async fn spa_index(State(state): State<AppState>) -> impl IntoResponse {
    let path = format!("{}/index.html", state.static_root);
    let html = tokio::fs::read_to_string(path)
        .await
        .unwrap_or_else(|_| "<!doctype html><title>notes.supply</title><body>missing build output</body>".into());
    Html(html)
}
