pub mod experience_detail;
use crate::components::HeaderWrap;
use crate::components::gallery::{Gallery, GalleryType};
use dioxus::prelude::*;

#[component]
pub fn Experience() -> Element {
    rsx! {
        ExperienceHeaderWrap {}
        Gallery { gallery_type: GalleryType::Experience, show_title: false }
    }
}

#[component]
fn ExperienceHeaderWrap() -> Element {
    rsx! {
        HeaderWrap {
            bg_image: "/img/work.jpg",
            min_height_class: "min-h-[650px]",
            pt_class: "pt-[250px]",
            div { class: "mx-auto w-full lg:w-1/2",
                h4 { class: "text-white text-lg font-normal", "YOU WANT TO KNOW" }
                h1 { class: "text-white pt-[10px] pb-[20px] tracking-[4px] text-[50px]", "WHERE I'VE BEEN" }
            }
        }
    }
}
