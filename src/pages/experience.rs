pub mod experience_detail;
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
        div { id: "work-wrap-non-bs",
            div { class: "container-lg",
                div { class: "row justify-content-center",
                    div { class: "col-lg-6",
                        h4 { "YOU WANT TO KNOW" }
                        h1 { "WHERE I'VE BEEN" }
                    
                    }
                }
            }
        }
    }
}
