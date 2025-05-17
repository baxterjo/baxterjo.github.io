use crate::components::page_not_found::PageNotFound;
use crate::content::SiteContent;
use crate::markdown::Markdown;
use dioxus::prelude::*;

#[component]
pub fn ExperienceDetail(name: String) -> Element {
    let content_sig = use_context::<Signal<SiteContent>>();
    let content = content_sig.read();

    let content_info = content.experience.get(&name);

    match content_info {
        Some(segment) => {
            let title = segment.config.title.as_deref().unwrap_or("");
            let md_content = &segment.markdown;
            let description = segment.config.description.as_deref().unwrap_or("");

            rsx! {
                ExperienceDetailHeaderWrap { title: "{title}", description: "{description}" }
                div { class: "container-lg",
                    div { class: "row justify-content-center mt-3",
                        div { class: "col-lg-8",
                            Markdown {
                                class: "content centered img-lg",
                                content: "{md_content}",
                            }
                        }
                    }
                }
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
fn ExperienceDetailHeaderWrap(
    title: ReadOnlySignal<String>,
    description: ReadOnlySignal<String>,
) -> Element {
    rsx! {
        div { id: "work-wrap-non-bs",
            div { class: "container-lg",
                div { class: "row justify-content-center",
                    div { class: "col-lg-6",
                        h1 { "{title}" }
                        h4 { "{description}" }
                    }
                }
            }
        }
    }
}
