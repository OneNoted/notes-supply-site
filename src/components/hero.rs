use dioxus::prelude::*;

use crate::components::TerminalIntro;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "hero-wrapper",
            class: "relative min-h-screen flex items-center justify-center overflow-hidden",
            "data-revealed": "false",

            canvas {
                id: "hero-particles",
                class: "absolute inset-0 w-full h-full pointer-events-none z-[1]",
                "data-hero-particles": true,
            }

            div { class: "absolute inset-0 radial-atmosphere dot-grid" }

            div { class: "relative z-10 w-full max-w-2xl px-6",
                div { class: "hero-stage",
                    div { class: "hero-overlay",
                        TerminalIntro {}
                    }

                    div { class: "hero-copy text-center",
                        h1 { class: "hero-content font-display text-6xl md:text-8xl font-extrabold tracking-tight leading-none mb-6",
                            "notes"
                        }

                        p {
                            class: "hero-content text-sm text-text-dim leading-relaxed max-w-md mx-auto mb-5",
                            style: "font-family: var(--font-mono)",
                            "software, systems, and field notes."
                            br {}
                            "Biting off more than I can chew since '07."
                        }

                        div {
                            class: "hero-content flex flex-col items-center gap-0.5",
                            style: "font-family: var(--font-mono)",
                            div { class: "text-[10px] text-text-muted tracking-wide",
                                "notes"
                                "["
                                span { style: "color: var(--color-accent)", "dot" }
                                "]"
                                "supply"
                            }
                            a {
                                href: "https://github.com/OneNoted",
                                target: "_blank",
                                rel: "noopener",
                                class: "block text-[10px] text-text-muted hover:text-accent transition-colors duration-fast tracking-wide",
                                "github"
                                "["
                                span { style: "color: var(--color-accent)", "dot" }
                                "]"
                                "com"
                                "["
                                span { style: "color: var(--color-accent)", "/" }
                                "]"
                                "onenoted"
                            }
                        }
                    }
                }
            }

            a {
                href: "#site-footer",
                class: "hero-content scroll-hint absolute bottom-10 left-1/2 -translate-x-1/2 z-10",
                span { class: "scroll-pill" }
            }
        }
    }
}
