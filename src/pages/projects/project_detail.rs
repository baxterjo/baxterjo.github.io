use dioxus::prelude::*;

use crate::components::page_not_found::PageNotFound;
use crate::content::SiteContent;
use crate::markdown::Markdown;

#[component]
fn ProjectDetail(title: ReadOnlySignal<String>, content: ReadOnlySignal<String>) -> Element {
    rsx! {
        ProjectDetailHeaderWrap { title: "{title}" }
        div { class: "container-lg",
            div { class: "row justify-content-center",
                div { class: "col-lg-8",
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
        div { id: "work-wrap-non-bs",
            div { class: "container-lg",
                div { class: "row justify-content-center",
                    div { class: "col-lg-6",
                        h1 { "{title}" }
                    
                    }
                }
            }
        }
    }
}
