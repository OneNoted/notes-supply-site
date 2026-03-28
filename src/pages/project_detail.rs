use dioxus::prelude::*;

use crate::client_api;

#[component]
pub fn ProjectDetailPage(slug: String) -> Element {
    let project = {
        let slug = slug.clone();
        use_resource(move || {
            let slug = slug.clone();
            async move { client_api::fetch_project(&slug).await }
        })
    };

    let readme = {
        let slug = slug.clone();
        use_resource(move || {
            let slug = slug.clone();
            async move { client_api::fetch_readme(&slug).await }
        })
    };

    rsx! {
        document::Title { "notes.supply // {slug}" }
        main { class: "min-h-screen bg-bg text-text font-body antialiased px-6 pt-24 pb-16",
            div { class: "max-w-4xl mx-auto",
                a {
                    href: "/projects",
                    class: "font-mono text-xs text-text-dim hover:text-text transition-colors duration-fast",
                    "$ cd projects"
                }

                div { class: "mt-6 mb-8",
                    match &*project.read() {
                        Some(Ok(item)) => rsx! {
                            div { class: "font-mono text-[10px] text-text-muted tracking-[0.2em] uppercase mb-3", "repository" }
                            h1 { class: "font-display text-5xl font-extrabold tracking-tight text-accent text-glow mb-3", "{item.name}" }
                            p { class: "text-sm text-text-dim max-w-3xl leading-relaxed mb-6",
                                "{item.description.clone().unwrap_or_else(|| \"No description available.\".into())}"
                            }
                            div { class: "flex flex-wrap items-center gap-3 text-[11px] font-mono text-text-muted",
                                span { "lang // {item.language.clone().unwrap_or_else(|| \"n/a\".into())}" }
                                span { "stars // {item.stars}" }
                                span { "forks // {item.forks}" }
                                a {
                                    href: "{item.html_url}",
                                    target: "_blank",
                                    rel: "noopener",
                                    class: "hover:text-accent transition-colors duration-fast",
                                    "open github"
                                }
                                if let Some(homepage) = item.homepage.clone().filter(|value| !value.is_empty()) {
                                    a {
                                        href: "{homepage}",
                                        target: "_blank",
                                        rel: "noopener",
                                        class: "hover:text-accent transition-colors duration-fast",
                                        "homepage"
                                    }
                                }
                            }
                        },
                        Some(Err(error)) => rsx! {
                            div { class: "terminal-panel p-3 font-mono text-sm text-red mb-6", "{error}" }
                        },
                        None => rsx! {
                            div { class: "terminal-panel p-3 font-mono text-sm text-text-dim mb-6", "loading..." }
                        },
                    }
                }

                div { class: "terminal-panel",
                    div { class: "terminal-panel-header font-mono text-[11px] text-text-muted tracking-wide", "README.md" }
                    div { class: "terminal-panel-body",
                        match &*readme.read() {
                            Some(Ok(text)) => rsx! {
                                pre { class: "term-fetch-output project-readme", "{text}" }
                            },
                            Some(Err(error)) => rsx! {
                                div { class: "font-mono text-sm text-red", "{error}" }
                            },
                            None => rsx! {
                                div { class: "font-mono text-sm text-text-dim", "fetching README..." }
                            },
                        }
                    }
                }
            }
        }
    }
}

