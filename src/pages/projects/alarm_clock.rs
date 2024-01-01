use dioxus::prelude::*;
// use dioxus_markdown::Markdown;

#[component]
pub fn AlarmClock(cx: Scope) -> Element {
    render! {
        h1 {
            "This is the alarm clock page."
        }
    }
}
