use crate::capitalize;
use crate::components::nav_bar::NavBar;
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::{debug, warn};

#[component]
pub fn Layout(cx: Scope) -> Element {
    let current_route: Route = use_route(&cx).unwrap();

    let route_str = current_route.to_string();
    let route_split: Vec<&str> = route_str.split("/").collect();
    debug!("Got string split: {route_split:?}");

    let title = if route_split[1] == "" {
        "Jordan Baxter".to_string()
    } else {
        format!("Jordan Baxter - {}", capitalize(route_split[1]))
    };

    let window = web_sys::window();
    if let Some(window) = window {
        if let Some(document) = window.document() {
            document.set_title(&title)
        } else {
            warn!("Couldn't get document to change document title.");
        }
    } else {
        warn!("Couldn't get window to change document title.");
    }

    render! {
        NavBar{}
        Outlet::<Route> {}
    }
}
