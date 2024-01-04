use crate::content::SiteContent;
use crate::markdown::Markdown;
use dioxus::prelude::*;

#[component]
pub fn AlarmClockWrap(cx: Scope) -> Element {
    log::debug!("Rendering alarm clock from markdown in dynamic site content...");

    let content = &*use_shared_state::<SiteContent>(cx).unwrap().read();
    let markdown_owned = content
        .projects
        .hardware
        .get("alarm_clock")
        .unwrap()
        .markdown
        .clone();
    render! {
        AlarmClock{md: markdown_owned}
    }
}

#[component]
pub fn AlarmClock(cx: Scope, md: String) -> Element {
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
                    Markdown{
                        class: "content centered img-fluid container",
                        content: "{md}"
                    }

                }
            }
        }
    }
}
