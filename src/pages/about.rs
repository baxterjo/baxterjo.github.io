use dioxus::prelude::*;

#[component]
pub fn About(cx: Scope) -> Element {
    render! {
        AboutHeaderWrap {}

    }
}

#[component]
fn AboutHeaderWrap(cx: Scope) -> Element {
    render! {
        div {
            id: "work-wrap-non-bs",
            div {
                class: "container-lg",
                div {
                    class: "row justify-content-center",
                    div {
                        class: "col-lg-6",
                        h4 {
                            "SO YOU'RE CURIOUS"
                        }
                        h1 {
                            "ABOUT ME"
                        }

                    }
                }
            }
        }
    }
}
