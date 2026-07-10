use crate::components::gallery::{Gallery, GalleryType};
use crate::components::{Container, HeaderWrap};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        HomePageHeaderWrap {}


        Gallery {
            max_cards: 3,
            gallery_type: GalleryType::Experience,
            show_title: false,
        }
        div { class: "bg-neutral-100",
            Container {
                div { class: "py-3 text-center",

                    h2 { "Personal Projects" }
                }
            }
        }
        Gallery {
            max_cards: 3,
            gallery_type: GalleryType::SoftwareProjects,
            show_title: true,
        }
        Gallery {
            max_cards: 3,
            gallery_type: GalleryType::HardwareProjects,
            show_title: true,
        }
    }
}

#[component]
fn HomePageHeaderWrap() -> Element {
    rsx! {
        HeaderWrap {
            bg_image: "/img/back.jpg",
            min_height_class: "min-h-[650px]",
            pt_class: "pt-[180px] md:pt-[250px]",
            h4 { class: "text-white text-lg font-normal", "HELLO, MY NAME IS" }
            h1 { class: "text-white pt-[10px] pb-[20px] tracking-[4px] text-[70px]", "JORDAN BAXTER" }
            h4 { class: "text-white text-lg font-normal",
                "FULL STACK IOT / NETWORKING ENGINEER"
                br {}
                "AND SYSTEMS ARCHITECT"
            }
        }
    }
}
