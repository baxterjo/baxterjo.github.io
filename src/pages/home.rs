use crate::components::{Gallery, GalleryCard, GalleryType};
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    render! {
        HomePageHeaderWrap {}
        div{
            class: "py-5 bg-light",
            div {
                class: "container",
                div {
                    class: "row centered",

                        h1 {
                            "Portfolio Overview"
                        }

                }
            }
        }

        Gallery{max_cards:3, gallery_type: GalleryType::Experience}
        Gallery{max_cards:3, gallery_type: GalleryType::SoftwareProjects}
        Gallery{max_cards:3, gallery_type: GalleryType::HardwareProjects}
    }
}

#[component]
fn HomePageHeaderWrap(cx: Scope) -> Element {
    render! {
        div {
            id: "home-page-header-wrap-non-bs",
            div {
                class: "container",
                div {
                    class: "row justify-content-center",
                    div {
                        class: "col-lg-6",
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

#[component]
fn ExperienceGallery(cx: Scope) -> Element {
    render! {
        div {
            class: "py-5 bg-light",
            div {
                class: "container",
                div {
                    class: "row centered",
                    h2 {
                        "Experience"
                    }
                }
                div {
                    class: "row",
                    div {
                        class: "card-group justify-content-center",
                        GalleryCard {
                            img_path: "img/experience/generac.png",
                            route_to: Route::PageNotFound { route:  vec!["experience/generac".to_string()]  },
                            description: "September 2021 - Present"
                        }
                        GalleryCard{
                            img_path:"img/experience/apricity.png",
                            route_to: Route::PageNotFound { route:  vec!["experience/apricity".to_string()]  },
                            description: "February 2018 - September 2021"
                        }

                    }

                }
            }
        }

    }
}
