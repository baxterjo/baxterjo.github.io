use crate::components::{layout::Layout, page_not_found::PageNotFound};
use crate::pages::{
    about::About,
    contact::Contact,
    education::Education,
    experience::{experience_detail::ExperienceDetail, Experience},
    home::Home,
    projects::{
        project_detail::{HardwareProjectDetail, SoftwareProjectDetail},
        ProjectsRoot,
    },
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/about", About)]
        About {},
        #[nest("/experience")]
            #[route("/")]
            Experience{},
            #[route("/experience/:name")]
            ExperienceDetail {name: String},
        #[end_nest]
        #[nest("/projects")]
            #[route("/")]
            ProjectsRoot{},
            #[route("/hardware/:name")]
            HardwareProjectDetail {name: String},
            #[route("/software/:name")]
            SoftwareProjectDetail {name: String},
        #[end_nest]
        #[route("/education", Education)]
        Education{},
        #[route("/contact", Contact)]
        Contact{},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
