use dioxus::prelude::*;

#[component]
pub fn ExperienceDetail(cx: Scope, name: String) -> Element {
    render! {
        h1 { "This is the experience page for {name}"  }
    }
}
