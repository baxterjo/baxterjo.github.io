use crate::components::gallery::{Gallery, GalleryType};
use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    render! {
        HomePageHeaderWrap {}
        div{
            class: "py-5 bg-light",
            div {
                class: "container-lg",
                div {
                    class: "row centered",

                        h1 {
                            "Portfolio Overview"
                        }

                }
            }
        }

        Gallery{max_cards:3, gallery_type: GalleryType::Experience, show_title: false}
        Gallery{max_cards:3, gallery_type: GalleryType::SoftwareProjects, show_title: true}
        Gallery{max_cards:3, gallery_type: GalleryType::HardwareProjects, show_title: true}
    }
}

#[component]
fn HomePageHeaderWrap(cx: Scope) -> Element {
    render! {
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
