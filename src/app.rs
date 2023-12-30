use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
// create a component that renders a div with the text "Hello, world!"
pub fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
