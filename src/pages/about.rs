use dioxus::prelude::*;

#[component]
pub fn About(cx: Scope) -> Element {
    render! {
        h1 { "This is the about page"  }
    }
}
