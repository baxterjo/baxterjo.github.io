use dioxus::prelude::*;

#[component]
pub(crate) fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}
