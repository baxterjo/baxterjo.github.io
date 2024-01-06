pub mod project_detail;
use dioxus::prelude::*;
use log::warn;

use crate::components::gallery::{Gallery, GalleryType};

#[component]
pub fn ProjectsRoot(cx: Scope) -> Element {
    render! {
        ProjectHeaderWrap {}
        Gallery{ gallery_type: GalleryType::SoftwareProjects}
        Gallery{ gallery_type: GalleryType::HardwareProjects}
    }
}

#[component]
fn ProjectHeaderWrap(cx: Scope) -> Element {
    render! {
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
