use dioxus::prelude::*;
use dioxus_markdown::Markdown;

#[component]
pub fn AlarmClock(cx: Scope) -> Element {
    render! {
        div {
            id: "work-wrap-non-bs",
            div {
                class: "container",
                div {
                    class: "row justify-content-center",
                    div {
                        class: "col-lg-6",
                        h1 {
                            "8-Bit Alarm Clock"
                        }

                    }
                }
            }
        }
        div {
            class: "container",
            div {
                class: "row justify-content-center",
                div {
                    class: "col-lg-8",
                    // Markdown{
                    //     class: "content centered img-fluid container",
                    //     content: include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/public/content/projects/alarm_clock/content.md"))
                    // }

                }
            }
        }
    }
}
