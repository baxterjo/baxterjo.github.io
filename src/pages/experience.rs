use dioxus::prelude::*;

pub mod experience_detail;

#[component]
pub fn Experience(cx: Scope) -> Element {
    render! {
        h1 { "This is the experience page"  }
    }
}
