use crate::components::gallery::{Gallery, GalleryType};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        HomePageHeaderWrap {}


        Gallery{max_cards:3, gallery_type: GalleryType::Experience, show_title: false}
        div{
            class: "bg-light",
            div {
                class: "container-lg",
                div {
                    class: "row centered py-3",

                        h2 {
                            "Personal Projects"
                        }

                }
            }
        }
        Gallery{max_cards:3, gallery_type: GalleryType::SoftwareProjects, show_title: true}
        Gallery{max_cards:3, gallery_type: GalleryType::HardwareProjects, show_title: true}
    }
}

#[component]
fn HomePageHeaderWrap() -> Element {
    rsx! {
        div {
            id: "home-page-header-wrap-non-bs",
            div {
                class: "container-lg",
                div {
                    class: "row justify-content-center",
                    div {
                        class: "col",
                        h4 {
                            "HELLO, MY NAME IS"
                        }
                        h1 {
                            "JORDAN BAXTER"
                        }
                        h4 {
                            "FULL STACK IOT ENGINEER"
                            br {}
                            "AND SYSTEMS ARCHITECT"
                        }
                    }
                }
            }
        }
    }
}
