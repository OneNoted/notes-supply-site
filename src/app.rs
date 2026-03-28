use dioxus::prelude::*;

use crate::components::{Footer, Hero};
use crate::pages::{ProjectDetailPage, ProjectsPage};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/projects")]
    Projects {},
    #[route("/project/:slug")]
    ProjectDetail { slug: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        document::Title { "notes.supply" }
        div { class: "bg-bg text-text font-body antialiased",
            Hero {}
            Footer {}
        }
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        div { class: "bg-bg text-text font-body antialiased",
            ProjectsPage {}
            Footer {}
        }
    }
}

#[component]
fn ProjectDetail(slug: String) -> Element {
    rsx! {
        div { class: "bg-bg text-text font-body antialiased",
            ProjectDetailPage { slug: slug.clone() }
            Footer {}
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    let requested = if route.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", route.join("/"))
    };

    rsx! {
        document::Title { "notes.supply // 404" }
        div { class: "min-h-screen bg-bg text-text font-body antialiased flex items-center justify-center px-6",
            div { class: "text-center",
                div { class: "text-5xl font-display font-extrabold text-accent text-glow mb-4", "404" }
                div { class: "font-mono text-sm text-text-dim mb-6", "no route for {requested}" }
                a {
                    href: "/",
                    class: "font-mono text-xs text-text-muted hover:text-text transition-colors duration-fast",
                    "$ cd /"
                }
            }
        }
    }
}

