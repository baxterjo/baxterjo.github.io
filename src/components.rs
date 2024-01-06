pub mod gallery;
pub mod layout;
pub mod nav_bar;
pub mod page_not_found;
use dioxus::prelude::*;

#[component]
pub fn Footer(cx: Scope) -> Element {
    render! {
        div {
            id: "copyrights",
            div {
                class: "container-lg",
                div {
                    class: "credits flex-row",
                    div {
                        a {
                            class: "flex-col",
                            href: "https://dioxuslabs.com/",
                            div {
                                class:"flex-shrink-0",
                                img {
                                    src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                                    alt:"Dioxus Labs Icon"
                                }
                            }
                            div {
                                class: "flex-grow-1 ms-3",
                                "Powered by Dioxus"
                            }
                        }
                    }
                    div {
                        class: "flex-col",
                        p {
                            class:"mt-2",
                            "© Styling copyrights "
                            strong {"Instant"}
                            " All Rights Reserved"
                        }
                        "Styling created with Instant template by "
                        a {
                            href:"https://templatemag.com/",
                            "TemplateMag"
                        }
                    }

                }
            }
        }
    }
}

#[component]
pub fn Social(cx: Scope) -> Element {
    render! {
        div {
            id: "social",
            div {
                class: "container-lg",
                div {
                    class:"row justify-content-center",
                    div {
                        class:"col-lg-2 centered",
                        a {
                            href:"http://www.linkedin.com/in/jordan-baxterece",
                            i {
                                class :"fa fa-linkedin"
                            }
                        }
                    }
                }
            }
        }
    }
}
