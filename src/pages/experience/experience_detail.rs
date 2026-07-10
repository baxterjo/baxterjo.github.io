use crate::components::page_not_found::PageNotFound;
use crate::components::{Container, HeaderWrap};
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
                Container {
                    div { class: "mt-3 flex justify-center",
                        div { class: "w-full lg:w-[66.6667%]",
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
        HeaderWrap {
            bg_image: "/img/work.jpg",
            min_height_class: "min-h-[650px]",
            pt_class: "pt-[250px]",
            div { class: "mx-auto w-full lg:w-1/2",
                h1 { class: "text-white pt-[10px] pb-[20px] tracking-[4px] text-[50px]", "{title}" }
                h4 { class: "text-white text-lg font-normal", "{description}" }
            }
        }
    }
}
