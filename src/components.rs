pub mod nav_bar;
pub mod page_not_found;

use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

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
