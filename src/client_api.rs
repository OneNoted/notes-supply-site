use crate::types::{GithubStats, Project, ProjectListItem};

#[derive(Debug, Clone, thiserror::Error)]
pub enum ClientApiError {
    #[error("{0}")]
    Message(String),
}

#[cfg(target_arch = "wasm32")]
fn api_origin() -> String {
    let Some(window) = web_sys::window() else {
        return "http://127.0.0.1:4000".into();
    };

    let location = window.location();
    let protocol = location.protocol().unwrap_or_else(|_| "http:".into());
    let hostname = location.hostname().unwrap_or_else(|_| "127.0.0.1".into());
    let port = location.port().unwrap_or_default();
    let origin = location
        .origin()
        .unwrap_or_else(|_| format!("{protocol}//{hostname}"));
    let is_localhost = matches!(hostname.as_str(), "localhost" | "127.0.0.1" | "::1");

    if is_localhost && port != "4000" {
        format!("{protocol}//{hostname}:4000")
    } else {
        origin
    }
}

#[cfg(target_arch = "wasm32")]
async fn get_json<T>(path: &str) -> Result<T, ClientApiError>
where
    T: serde::de::DeserializeOwned,
{
    let url = format!("{}/api/{}", api_origin(), path.trim_start_matches('/'));
    let response = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|error| ClientApiError::Message(format!("request failed: {error}")))?;

    if !response.ok() {
        return Err(ClientApiError::Message(format!(
            "request failed with HTTP {}",
            response.status()
        )));
    }

    response
        .json::<T>()
        .await
        .map_err(|error| ClientApiError::Message(format!("invalid response: {error}")))
}

#[cfg(target_arch = "wasm32")]
async fn get_text(path: &str) -> Result<String, ClientApiError> {
    let url = format!("{}/api/{}", api_origin(), path.trim_start_matches('/'));
    let response = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|error| ClientApiError::Message(format!("request failed: {error}")))?;

    if !response.ok() {
        return Err(ClientApiError::Message(format!(
            "request failed with HTTP {}",
            response.status()
        )));
    }

    response
        .text()
        .await
        .map_err(|error| ClientApiError::Message(format!("invalid response: {error}")))
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_projects() -> Result<Vec<ProjectListItem>, ClientApiError> {
    get_json("projects").await
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_project(slug: &str) -> Result<Project, ClientApiError> {
    get_json(&format!("project/{slug}")).await
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_readme(slug: &str) -> Result<String, ClientApiError> {
    get_text(&format!("readme/{slug}")).await
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_stats() -> Result<GithubStats, ClientApiError> {
    get_json("stats").await
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_projects() -> Result<Vec<ProjectListItem>, ClientApiError> {
    Err(ClientApiError::Message("client API unavailable on native builds".into()))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_project(_slug: &str) -> Result<Project, ClientApiError> {
    Err(ClientApiError::Message("client API unavailable on native builds".into()))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_readme(_slug: &str) -> Result<String, ClientApiError> {
    Err(ClientApiError::Message("client API unavailable on native builds".into()))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_stats() -> Result<GithubStats, ClientApiError> {
    Err(ClientApiError::Message("client API unavailable on native builds".into()))
}
