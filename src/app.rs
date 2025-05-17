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
        //document::Script { src: "https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/js/bootstrap.bundle.min.js" }
        //document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css" }
        //document::Stylesheet { href: "https://fonts.googleapis.com/css?family=Montserrat:300,400,500,700" }
        //document::Stylesheet { href: asset!("public/css/style.css") }
        //document::Stylesheet { href: asset!("public/css/font-awesome/css/font-awesome.min.css") }
        Router::<Route> {}
    }
}
