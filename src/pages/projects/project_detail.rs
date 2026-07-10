use dioxus::prelude::*;

use crate::components::page_not_found::PageNotFound;
use crate::components::{Container, HeaderWrap};
use crate::content::SiteContent;
use crate::markdown::Markdown;

#[component]
fn ProjectDetail(title: ReadOnlySignal<String>, content: ReadOnlySignal<String>) -> Element {
    rsx! {
        ProjectDetailHeaderWrap { title: "{title}" }
        Container {
            div { class: "flex justify-center",
                div { class: "w-full lg:w-[66.6667%]",
                    Markdown {
                        class: "content centered img-lg container",
                        content: "{content}",
                    }
                }
            }
        }
    }
}

#[component]
pub fn HardwareProjectDetail(name: String) -> Element {
    let content_sig = use_context::<Signal<SiteContent>>();
    let content = content_sig.read();

    let content_info = content.hardware_projects.get(&name);

    match content_info {
        Some(segment) => {
            let title = segment.config.title.as_deref().unwrap_or("");
            let md_content = &segment.markdown;
            rsx! {
                ProjectDetail { title: "{title}", content: "{md_content}" }
            }
        }
        None => {
            rsx! {
                PageNotFound { route: vec![name.clone()] }
            }
        }
    }
}

#[component]
pub fn SoftwareProjectDetail(name: String) -> Element {
    let content_sig = use_context::<Signal<SiteContent>>();
    let content = content_sig.read();

    let content_info = content.software_projects.get(&name);

    match content_info {
        Some(segment) => {
            let title = segment.config.title.as_deref().unwrap_or("");
            let md_content = &segment.markdown;
            rsx! {
                ProjectDetail { title: "{title}", content: "{md_content}" }
            }
        }
        None => {
            rsx! {
                PageNotFound { route: vec![name.clone()] }
            }
        }
    }
}

#[component]
fn ProjectDetailHeaderWrap(title: ReadOnlySignal<String>) -> Element {
    rsx! {
        HeaderWrap {
            bg_image: "/img/work.jpg",
            min_height_class: "min-h-[650px]",
            pt_class: "pt-[250px]",
            div { class: "mx-auto w-full lg:w-1/2",
                h1 { class: "text-white pt-[10px] pb-[20px] tracking-[4px] text-[50px]", "{title}" }
            }
        }
    }
}
