pub mod project_detail;
use dioxus::prelude::*;
use log::warn;

use crate::components::gallery::{Gallery, GalleryCard, GalleryCardArgs, GalleryType};
use crate::router::Route;

#[component]
pub fn ProjectsRoot(cx: Scope) -> Element {
    // let window = web_sys::window();
    // if let Some(window) = window {
    //     if let Some(document) = window.document() {
    //         document.set_title("Jordan Baxter - Projects")
    //     } else {
    //         warn!("Couldn't get document to change document title.");
    //     }
    // } else {
    //     warn!("Couldn't get window to change document title.");
    // }
    render! {
        ProjectHeaderWrap {}
        Gallery{ gallery_type: GalleryType::SoftwareProjects}
        Gallery{ gallery_type: GalleryType::HardwareProjects}
    }
}

#[component]
fn ProjectHeaderWrap(cx: Scope) -> Element {
    render! {
        div {
            id: "work-wrap-non-bs",
            div {
                class: "container-fluid",
                div {
                    class: "row justify-content-center",
                    div {
                        class: "col-lg-6",
                        h4 {
                            "TAKE A LOOK AT MY"
                        }
                        h1 {
                            "PERSONAL PROJECTS"
                        }

                    }
                }
            }
        }
    }
}

#[component]
pub fn SoftwareGallery(cx: Scope, max_cards: Option<usize>) -> Element {
    let max_cards = max_cards.unwrap_or(usize::MAX);
    let mut cards: Vec<GalleryCardArgs> = vec![
        GalleryCardArgs {
            title: "Antenna Simulator",
            img_path: "img/portfolio/antennasimulator.gif",
            route_to: Route::PageNotFound { route: vec!["projects/antenna-simulator".to_string()] },
            description: "An antenna simulation application written in python that uses the tkinter toolkit to render a GUI."
        }
    ];

    cards.truncate(max_cards);

    let cards_rendered = cards.iter().map(|card| {
        render! {
            GalleryCard {
                title: card.title,
                img_path: card.img_path,
                route_to: card.route_to.clone(),
                description:card.description
            }
        }
    });

    render! {
        div {
            class: "py-5 bg-light",
            div {
                class: "container-fluid",
                div {
                    class: "row centered",
                    h2 {
                        "Software"
                    }
                }
                div {
                    class: "row justify-content-center",
                    div {
                        class: "card-group justify-content-center",
                        cards_rendered

                    }
                }
            }
        }

    }
}
