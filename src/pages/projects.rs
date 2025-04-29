pub mod project_detail;
use dioxus::prelude::*;

use crate::components::gallery::{Gallery, GalleryType};

#[component]
pub fn ProjectsRoot() -> Element {
    rsx! {
        ProjectHeaderWrap {}
        Gallery{ gallery_type: GalleryType::SoftwareProjects, show_title: true}
        Gallery{ gallery_type: GalleryType::HardwareProjects, show_title: true}
    }
}

#[component]
fn ProjectHeaderWrap() -> Element {
    rsx! {
        div {
            id: "work-wrap-non-bs",
            div {
                class: "container-lg",
                div {
                    class: "row justify-content-center",
                    div {
                        class: "col-lg-6",
                        h4 {
                            "TAKE A LOOK AT MY"
                        }
                        h1 {
                            "PERSONAL PROJECTS"
                        }

                    }
                }
            }
        }
    }
}
