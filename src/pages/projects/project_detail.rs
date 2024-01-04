use dioxus::prelude::*;

use crate::components::page_not_found::PageNotFound;
use crate::components::{GalleryCard, GalleryCardArgs};
use crate::content::SiteContent;
use crate::router::Route;
use dioxus_markdown::Markdown;

#[component]
pub fn ProjectDetail<'a>(cx: Scope, title: &'a str, content: &'a str) -> Element {
    render! {
        ProjectDetailHeaderWrap{ title: "{title}"}
        div {
            class: "container",
            div {
                class: "row justify-content-center",
                div {
                    class: "col-lg-8",
                    Markdown{
                        class: "content centered img-fluid container",
                        content: "{content}"
                    }

                }
            }
        }
    }
}

#[component]
pub fn HardwareProjectDetail(cx: Scope, name: String) -> Element {
    let content = &*use_shared_state::<SiteContent>(cx).unwrap().read();
    let content_info = content.projects.hardware.get(name);

    match content_info {
        Some(segment) => {
            let title = segment.config.title.as_deref().unwrap_or("");
            let md_content = &segment.markdown;
            render! {
                ProjectDetail {
                    title: "{title}",
                    content:"{md_content}"
                }
            }
        }
        None => {
            render! {
                PageNotFound{route: vec![name.clone()]}
            }
        }
    }
}

#[component]
pub fn SoftwareProjectDetail(cx: Scope, name: String) -> Element {
    let content = &*use_shared_state::<SiteContent>(cx).unwrap().read();
    render! {
        ProjectDetailHeaderWrap{ title: "{name}"}
    }
}

#[component]
fn ProjectDetailHeaderWrap<'a>(cx: Scope, title: &'a str) -> Element {
    render! {
        div {
            id: "work-wrap-non-bs",
            div {
                class: "container",
                div {
                    class: "row justify-content-center",
                    div {
                        class: "col-lg-6",
                        h1 {
                            "{title}"
                        }

                    }
                }
            }
        }
    }
}
