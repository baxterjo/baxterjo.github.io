pub mod nav_bar;
pub mod page_not_found;

use std::collections::HashMap;

use crate::content::{ContentSegment, SiteContent};
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum GalleryType {
    Experience,
    HardwareProjects,
    SoftwareProjects,
}
#[derive(Debug)]
pub struct GalleryCardArgs<'a> {
    pub title: &'a str,
    pub img_path: &'a str,
    pub route_to: Route,
    pub description: &'a str,
}

#[component]
pub fn GalleryCard<'a>(
    cx: Scope,
    title: Option<&'a str>,
    img_path: &'a str,
    route_to: Route,
    description: Option<&'a str>,
) -> Element {
    render! {
        div {
            class: "col-lg-4 col-md-4 col-sm-4 gallery",
            div {
                class: "card bg-light text-white",
                img {
                    class: "card-img",
                    src:*img_path
                }
                Link {
                    class: "card-img-overlay",
                    to: route_to.clone(),
                    h5 {
                        class: "card-title",
                        title
                    }
                    p {
                        class: "card-text",
                        description
                    }
                }
            }
        }
    }
}

#[component]
pub fn Gallery(cx: Scope, max_cards: Option<usize>, gallery_type: GalleryType) -> Element {
    let content = &*use_shared_state::<SiteContent>(cx).unwrap().read();

    let max_cards = max_cards.unwrap_or(usize::MAX);

    let (gallery_title, content_itr) = match gallery_type {
        GalleryType::Experience => ("Experience", content.experience.iter().enumerate()),
        GalleryType::HardwareProjects => ("Hardware", content.projects.hardware.iter().enumerate()),
        GalleryType::SoftwareProjects => ("Software", content.projects.software.iter().enumerate()),
    };

    let mut cards: Vec<GalleryCardArgs> = vec![];
    for (itr, (id, info)) in content_itr {
        if itr < max_cards {
            let title = info.config.title.as_deref().unwrap_or("");
            let img_path = &info.config.thumbnail;

            let route_to = match gallery_type {
                GalleryType::Experience => Route::ExperienceDetail { name: id.clone() },
                GalleryType::HardwareProjects => Route::HardwareProjectDetail { name: id.clone() },
                GalleryType::SoftwareProjects => Route::SoftwareProjectDetail { name: id.clone() },
            };

            Route::HardwareProjectDetail { name: id.clone() };
            let description = info.config.description.as_deref().unwrap_or("");

            cards.push(GalleryCardArgs {
                title,
                img_path,
                route_to,
                description,
            });
        } else {
            break;
        }
    }

    let cards_rendered = cards.iter().map(|card| {
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
                        "{gallery_title}"
                    }
                }
                div {
                    class: "row justify-content-center",
                    div {
                        class: "card-deck",
                        cards_rendered

                    }
                }
            }
        }

    }
}
