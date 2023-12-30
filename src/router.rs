use crate::components::{self, page_not_found::PageNotFound};
use crate::pages::home::Home;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(components::NavBar)]
        #[route("/")]
        Home {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
