use crate::components::{nav_bar::NavBar, page_not_found::PageNotFound};
use crate::pages::{
    about::About,
    contact::Contact,
    education::Education,
    experience::Experience,
    home::Home,
    projects::{alarm_clock::AlarmClock, Projects},
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug)]
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
        #[route("/projects/alarm-clock")]
        AlarmClock {},
        #[route("/education", Education)]
        Education{},
        #[route("/contact", Contact)]
        Contact{},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
