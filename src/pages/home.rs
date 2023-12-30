use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    render! {
        h1 { "This is the home page" }
    }
}
