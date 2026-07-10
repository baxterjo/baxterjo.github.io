use crate::components::{Container, HeaderWrap};
use crate::markdown::Markdown;
use dioxus::prelude::*;

const ABOUT_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/site_content/about.md"
));

#[component]
pub fn About() -> Element {
    rsx! {
        AboutHeaderWrap {}
        Container {
            div { class: "mt-5 flex justify-center",
                div { class: "w-full lg:w-[41.6667%]",
                    img { class: "h-auto max-w-full", src: "/img/about/headshot.jpeg" }
                }

            }
            div { class: "my-3 flex justify-center",
                div { class: "w-full lg:w-[66.6667%]",
                    Markdown {
                        class: "content centered img-lg container",
                        content: "{ABOUT_MD}",
                    }

                }
            }
        }
    }
}

#[component]
fn AboutHeaderWrap() -> Element {
    rsx! {
        HeaderWrap {
            bg_image: "/img/work.jpg",
            min_height_class: "min-h-[650px]",
            pt_class: "pt-[250px]",
            h4 { class: "text-white text-lg font-normal", "SO YOU'RE CURIOUS" }
            h1 { class: "text-white pt-[10px] pb-[20px] tracking-[4px] text-[50px]", "ABOUT ME" }

        }
    }
}
