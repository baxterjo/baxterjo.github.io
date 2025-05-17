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
        div { class: "container-lg",
            div { class: "row justify-content-center mt-5",
                div { class: "col-lg-5",
                    img {
                        class: "img-fluid",
                        src: "/assets/img/about/headshot_cropped.jpg",
                    }
                
                }
            }
            div { class: "row justify-content-center my-3",
                div { class: "col-lg-8",
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
        div { id: "work-wrap-non-bs",
            div { class: "container-lg",
                div { class: "row justify-content-center",
                    div { class: "col-lg-6",
                        h4 { "SO YOU'RE CURIOUS" }
                        h1 { "ABOUT ME" }
                    
                    }
                }
            }
        }
    }
}
