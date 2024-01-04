pub mod alarm_clock;
pub mod project_detail;
use dioxus::prelude::*;

use crate::components::{GalleryCard, GalleryCardArgs};
use crate::content::SiteContent;
use crate::router::Route;

#[component]
pub fn ProjectsRoot(cx: Scope) -> Element {
    render! {
        ProjectHeaderWrap {}
        HardwareGallery {}
    }
}

#[component]
fn ProjectHeaderWrap(cx: Scope) -> Element {
    render! {
        div {
            id: "work-wrap-non-bs",
            div {
                class: "container",
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
pub fn HardwareGallery(cx: Scope, max_cards: Option<usize>) -> Element {
    let content = &*use_shared_state::<SiteContent>(cx).unwrap().read();

    let max_cards = max_cards.unwrap_or(usize::MAX);

    let mut hardware_cards: Vec<GalleryCardArgs> = vec![];

    for (itr, (id, info)) in content.projects.hardware.iter().enumerate() {
        if itr < max_cards {
            let title = info.config.title.as_deref().unwrap_or("");
            let img_path = &info.config.thumbnail;
            let route_to = Route::HardwareProjectDetail { name: id.clone() };
            let description = info.config.description.as_deref().unwrap_or("");

            hardware_cards.push(GalleryCardArgs {
                title,
                img_path,
                route_to,
                description,
            });
        } else {
            break;
        }
    }

    let hardware_cards_rendered = hardware_cards.iter().map(|card| {
        render! {
            GalleryCard{
                title:"{card.title}",
                img_path:"{card.img_path}",
                route_to:card.route_to.clone(),
                description: "{card.description}"
            }
        }
    });

    render! {
        div {
            class: "py-5 bg-light",
            div {
                class: "container",
                div {
                    class: "row centered",
                    h2 {
                        "Hardware"
                    }
                }
                div {
                    class: "row justify-content-center",
                    div {
                        class: "card-group",
                        hardware_cards_rendered

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
                class: "container",
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
