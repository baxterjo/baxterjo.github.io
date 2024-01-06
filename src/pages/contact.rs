use crate::components::gallery::{Gallery, GalleryType};
use dioxus::prelude::*;

#[component]
pub fn Contact(cx: Scope) -> Element {
    render! {
        ContactHeaderWrap{}
        div {
            class: "py-5 bg-light",
            div {
                class: "container-lg",
                div {
                    class: "row centered",
                    div {
                        class: "ratio"
                    }
                    iframe {
                        src:"https://docs.google.com/forms/d/e/1FAIpQLSfckres6rZnrzLDyTPWQpcKaKPLlLhG04FEItohIIVo1USyyg/viewform?embedded=true",
                        width:"640", height:"959",
                        frame_border:"0", margin_height:"0", margin_width:"0",
                        "Loading…"
                    }
                }

            }
        }
    }
}

#[component]
fn ContactHeaderWrap(cx: Scope) -> Element {
    render! {
        div {
            id: "contactwrap",
            div {
                class: "container-lg",
                div {
                    class: "row justify-content-center",
                    div {
                        class: "col-lg-6",
                        h4 {
                            "INTERESTED IN WORKING WITH ME?"
                        }
                        h1 {
                            "LET'S GET IN TOUCH"
                        }

                    }
                }
            }
        }
    }
}

// <iframe src="https://docs.google.com/forms/d/e/1FAIpQLSfckres6rZnrzLDyTPWQpcKaKPLlLhG04FEItohIIVo1USyyg/viewform?embedded=true" width="640" height="824" frameborder="0" marginheight="0" marginwidth="0">Loading…</iframe>
