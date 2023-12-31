use crate::components::{page_not_found::PageNotFound, NavBar};
use crate::pages::{
    about::About, contact::Contact, education::Education, experience::Experience, home::Home,
    projects::Projects,
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/about", About)]
        About {},
        #[route("/experience", Experience)]
        Experience{},
        #[route("/projects", Projects)]
        Projects{},
        #[route("/education", Education)]
        Education{},
        #[route("/contact", Contact)]
        Contact{},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
