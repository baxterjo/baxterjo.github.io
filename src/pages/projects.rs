pub mod project_detail;
use dioxus::prelude::*;

use crate::components::HeaderWrap;
use crate::components::gallery::{Gallery, GalleryType};

#[component]
pub fn ProjectsRoot() -> Element {
    rsx! {
        ProjectHeaderWrap {}
        Gallery { gallery_type: GalleryType::SoftwareProjects, show_title: true }
        Gallery { gallery_type: GalleryType::HardwareProjects, show_title: true }
    }
}

#[component]
fn ProjectHeaderWrap() -> Element {
    rsx! {
        HeaderWrap {
            bg_image: "/img/work.jpg",
            min_height_class: "min-h-[650px]",
            pt_class: "pt-[250px]",
            div { class: "mx-auto w-full lg:w-1/2",
                h4 { class: "text-white text-lg font-normal", "TAKE A LOOK AT MY" }
                h1 { class: "text-white pt-[10px] pb-[20px] tracking-[4px] text-[50px]", "PERSONAL PROJECTS" }
            }
        }
    }
}
