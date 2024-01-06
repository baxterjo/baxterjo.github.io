use crate::content::{ContentSegment, SiteContent};
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::debug;
use toml::value::Date;

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
pub fn Gallery(
    cx: Scope,
    max_cards: Option<usize>,
    gallery_type: GalleryType,
    show_title: bool,
) -> Element {
    let content = &*use_shared_state::<SiteContent>(cx).unwrap().read();

    let max_cards = max_cards.unwrap_or(usize::MAX);

    let (gallery_title, mut gallery_content) = match gallery_type {
        GalleryType::Experience => (
            "Experience and Education",
            content
                .experience
                .iter()
                .collect::<Vec<(&String, &ContentSegment)>>(),
        ),
        GalleryType::HardwareProjects => ("Hardware", content.hardware_projects.iter().collect()),
        GalleryType::SoftwareProjects => ("Software", content.software_projects.iter().collect()),
    };

    gallery_content.sort_by_key(|(_id, info)| {
        let mut key: Date = info.config.date_added.date.unwrap();
        key.day -= info.config.priority_level as u8;
        key
    });

    gallery_content.reverse();

    let mut cards: Vec<GalleryCardArgs> = vec![];
    for (itr, (id, info)) in gallery_content.into_iter().enumerate() {
        if itr < max_cards {
            let title = if *show_title {
                info.config.title.as_deref().unwrap_or("")
            } else {
                ""
            };
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

    debug!("Making gallery {gallery_type:#?} with cards: {cards:#?}");

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
                class: "container-lg",
                div {
                    class: "row centered",
                    h2 {
                        "{gallery_title}"
                    }
                }
                div {
                    class: "row",
                    div {
                        class: "card-group justify-content-center",
                        cards_rendered

                    }
                }


            }
        }

    }
}

#[component]
fn GalleryCard<'a>(
    cx: Scope,
    title: Option<&'a str>,
    img_path: &'a str,
    route_to: Route,
    description: Option<&'a str>,
) -> Element {
    render! {
        div {
            class: "col-md-4 col-sm-6",
            div {
                class: "card h-100 bg-light text-white mx-1",
                img {
                    class: "card-img h-100",
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
