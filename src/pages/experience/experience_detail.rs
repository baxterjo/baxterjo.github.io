use crate::components::page_not_found::PageNotFound;
use crate::content::SiteContent;
use crate::markdown::Markdown;
use dioxus::prelude::*;

#[component]
pub fn ExperienceDetail(cx: Scope, name: String) -> Element {
    let content = &*use_shared_state::<SiteContent>(cx).unwrap().read();
    let content_info = content.experience.get(name);

    match content_info {
        Some(segment) => {
            let title = segment.config.title.as_deref().unwrap_or("");
            let md_content = &segment.markdown;
            let description = segment.config.description.as_deref().unwrap_or("");

            render! {
                ExperienceDetailHeaderWrap{ title: "{title}", description: "{description}"}
                div {
                    class: "container-lg",
                    div {
                        class: "row justify-content-center",
                        div {
                            class: "col-lg-8",
                            Markdown{
                                class: "content centered img-lg container-lg",
                                content: "{md_content}"
                            }

                        }
                    }
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
fn ExperienceDetailHeaderWrap<'a>(cx: Scope, title: &'a str, description: &'a str) -> Element {
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
                        h4 {
                            "{description}"
                        }

                    }
                }
            }
        }
    }
}
