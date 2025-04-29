use crate::content::SiteContent;
use crate::router::Route;
use dioxus::prelude::*;
use log::info;

// create a component that renders a div with the text "Hello, world!"
pub fn app() -> Element {
    let site_content = SiteContent::new();
    info!("Initialized site with content: {site_content:#?}");
    use_context_provider(|| Signal::new(site_content));
    rsx! {
        Router::<Route> {}
    }
}
