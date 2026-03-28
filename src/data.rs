use crate::types::Project;

pub fn fallback_projects() -> Vec<Project> {
    vec![
        Project {
            name: "whispers".into(),
            slug: "whispers".into(),
            description: Some("Speech and transcription experiments in Rust.".into()),
            language: Some("Rust".into()),
            stars: 0,
            forks: 0,
            topics: vec!["rust".into(), "speech".into(), "cli".into()],
            html_url: "https://github.com/OneNoted/whispers".into(),
            homepage: None,
            updated_at: "2026-03-15T12:00:00Z".into(),
        },
        Project {
            name: "apollo-manager-rs".into(),
            slug: "apollo-manager-rs".into(),
            description: Some("Dioxus-powered control surface for a homelab.".into()),
            language: Some("Rust".into()),
            stars: 0,
            forks: 0,
            topics: vec!["rust".into(), "dioxus".into(), "homelab".into()],
            html_url: "https://github.com/OneNoted/apollo-manager-rs".into(),
            homepage: None,
            updated_at: "2026-03-15T12:00:00Z".into(),
        },
    ]
}

