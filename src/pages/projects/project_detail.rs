use dioxus::prelude::*;

use crate::components::page_not_found::PageNotFound;
use crate::content::SiteContent;
use crate::markdown::Markdown;

#[component]
fn ProjectDetail<'a>(cx: Scope, title: &'a str, content: &'a str) -> Element {
    render! {
        ProjectDetailHeaderWrap{ title: "{title}"}
        div {
            class: "container-lg",
            div {
                class: "row justify-content-center",
                div {
                    class: "col-lg-8",
                    Markdown{
                        class: "content centered img-lg container",
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
    let content_info = content.hardware_projects.get(name);

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
    let content_info = content.software_projects.get(name);

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
fn ProjectDetailHeaderWrap<'a>(cx: Scope, title: &'a str) -> Element {
    render! {
        div {
            id: "work-wrap-non-bs",
            div {
                class: "container-lg",
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
