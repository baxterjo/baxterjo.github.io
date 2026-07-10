use crate::components::{Container, HeaderWrap};
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        ContactHeaderWrap {}
        div { class: "bg-neutral-100 py-5",
            Container {
                div { class: "text-center",
                    p {
                        "Fill out this form or check me out on "
                        a { href: "https://www.linkedin.com/in/baxterjo", "Linkedin" }
                    }
                    iframe {
                        src: "https://docs.google.com/forms/d/e/1FAIpQLSfckres6rZnrzLDyTPWQpcKaKPLlLhG04FEItohIIVo1USyyg/viewform?embedded=true",
                        width: "640",
                        height: "959",
                        frame_border: "0",
                        margin_height: "0",
                        margin_width: "0",
                        "Loading…"
                    }
                }
            }
        }
    }
}

#[component]
fn ContactHeaderWrap() -> Element {
    rsx! {
        HeaderWrap {
            bg_image: "/img/contact.jpg",
            min_height_class: "min-h-[650px]",
            pt_class: "pt-[250px]",
            div { class: "mx-auto w-full lg:w-1/2",
                h4 { class: "text-white font-normal", "INTERESTED IN WORKING WITH ME?" }
                h1 { class: "text-white pt-[10px] pb-[20px] tracking-[4px] text-[60px]", "LET'S GET IN TOUCH" }
            }
        }
    }
}

// <iframe src="https://docs.google.com/forms/d/e/1FAIpQLSfckres6rZnrzLDyTPWQpcKaKPLlLhG04FEItohIIVo1USyyg/viewform?embedded=true" width="640" height="824" frameborder="0" marginheight="0" marginwidth="0">Loading…</iframe>
