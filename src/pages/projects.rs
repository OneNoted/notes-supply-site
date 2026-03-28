use dioxus::prelude::*;

use crate::client_api;

#[component]
pub fn ProjectsPage() -> Element {
    let projects = use_resource(move || async move { client_api::fetch_projects().await });

    rsx! {
        document::Title { "notes.supply // projects" }
        main { class: "min-h-screen bg-bg text-text font-body antialiased px-6 pt-24 pb-16",
            div { class: "max-w-4xl mx-auto",
                a {
                    href: "/",
                    class: "font-mono text-xs text-text-dim hover:text-text transition-colors duration-fast",
                    "$ cd .."
                }

                div { class: "mt-6 mb-12",
                    div { class: "font-mono text-[10px] text-text-muted tracking-[0.2em] uppercase mb-3", "directory" }
                    h1 { class: "font-display text-5xl font-extrabold tracking-tight text-accent text-glow mb-3", "projects" }
                    p { class: "text-sm text-text-dim max-w-md leading-relaxed", "Live repositories pulled from GitHub." }
                }

                match &*projects.read() {
                    Some(Ok(items)) => rsx! {
                        div { class: "grid md:grid-cols-2 gap-4",
                            for item in items {
                                a {
                                    key: "{item.slug}",
                                    href: "/project/{item.slug}",
                                    class: "terminal-panel block p-3",
                                    div { class: "flex items-start justify-between gap-3 mb-3",
                                        div {
                                            h2 { class: "font-mono text-sm text-accent mb-2 truncate", "{item.name}" }
                                            p { class: "text-xs text-text-dim line-clamp-2", "{item.language}" }
                                        }
                                        span { class: "font-mono text-[10px] text-text-muted", "★ {item.stars}" }
                                    }
                                    div { class: "font-mono text-[10px] text-text-muted", "{item.updated}" }
                                }
                            }
                        }
                    },
                    Some(Err(error)) => rsx! {
                        div { class: "terminal-panel p-3 font-mono text-sm text-red", "{error}" }
                    },
                    None => rsx! {
                        div { class: "terminal-panel p-3 font-mono text-sm text-text-dim", "loading..." }
                    },
                }
            }
        }
    }
}

