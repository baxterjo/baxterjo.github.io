pub mod alarm_clock;
use dioxus::prelude::*;

use crate::components::{GalleryCard, GalleryCardArgs};
use crate::router::Route;

#[component]
pub fn Projects(cx: Scope) -> Element {
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
    let max_cards = max_cards.unwrap_or(usize::MAX);
    let mut hardware_cards: Vec<GalleryCardArgs> = vec![
        GalleryCardArgs {
        title: "8-Bit Alarm Clock",
        img_path: "img/portfolio/clock1.jpg",
        route_to: Route::AlarmClock {},
        description: "An alarm clock made from scratch with an 8-bit AVR development board and jellybean parts."
    },
    GalleryCardArgs {
        title: "8-Bit Alarm Clock",
        img_path: "img/portfolio/clock1.jpg",
        route_to: Route::AlarmClock {},
        description: "An alarm clock made from scratch with an 8-bit AVR development board and jellybean parts."
    },
    GalleryCardArgs {
        title: "8-Bit Alarm Clock",
        img_path: "img/portfolio/clock1.jpg",
        route_to: Route::AlarmClock {},
        description: "An alarm clock made from scratch with an 8-bit AVR development board and jellybean parts."
    },
    GalleryCardArgs {
        title: "8-Bit Alarm Clock",
        img_path: "img/portfolio/clock1.jpg",
        route_to: Route::AlarmClock {},
        description: "An alarm clock made from scratch with an 8-bit AVR development board and jellybean parts."
    }
    ];

    hardware_cards.truncate(max_cards);

    let hardware_cards_rendered = hardware_cards.iter().map(|card| {
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
