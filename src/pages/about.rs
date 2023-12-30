use dioxus::prelude::*;

#[component]
pub fn About(cx: Scope) -> Element {
    render! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}
