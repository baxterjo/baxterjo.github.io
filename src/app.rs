use crate::content::SiteContent;
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::{debug, info};
// create a component that renders a div with the text "Hello, world!"
pub fn app(cx: Scope) -> Element {
    let site_content = SiteContent::new();
    info!("Initialized site with content: {site_content:#?}");
    use_shared_state_provider(cx, move || site_content);
    render! {
        Router::<Route> {}
    }
}
