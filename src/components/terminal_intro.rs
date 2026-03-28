use dioxus::prelude::*;

#[component]
pub fn TerminalIntro() -> Element {
    rsx! {
        div { class: "term-shell",
            div { class: "term-window", "data-terminal-window": true,
                div { class: "term-titlebar",
                    span { class: "term-title", "notes@notes.supply" }
                }
                div { class: "term-body font-mono text-xs", "data-terminal-body": true,
                    div { "data-terminal-history": true }
                    div { class: "term-line", "data-terminal-input-line": true,
                        span { class: "text-accent", "$" }
                        span { class: "term-input-area",
                            span { "data-terminal-input": true }
                            span { class: "term-solid", "data-terminal-hint-solid": true }
                            span { class: "term-ghost", "data-terminal-hint-ghost": true }
                            span { class: "terminal-cursor" }
                        }
                    }
                }
            }

            button {
                r#type: "button",
                class: "term-replay hidden font-mono text-base text-text-dim hover:text-text transition-colors duration-fast",
                "data-terminal-replay": true,
                span { class: "text-accent", "$" }
                span { style: "display:inline-block;width:4px" }
                "whoami"
                span { class: "terminal-cursor" }
            }
        }
    }
}
