use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { id: "site-footer", class: "border-t border-border py-12 px-6",
            div { class: "max-w-4xl mx-auto flex items-center justify-between text-[11px] font-mono text-text-muted",
                span { "notes.supply // 2026" }
                div { class: "flex items-center gap-4",
                    a {
                        href: "https://github.com/OneNoted",
                        target: "_blank",
                        rel: "noopener",
                        class: "hover:text-accent transition-colors duration-fast",
                        "GitHub"
                    }
                }
            }
        }
    }
}

