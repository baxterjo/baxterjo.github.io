pub mod gallery;
pub mod layout;
pub mod nav_bar;
pub mod page_not_found;
use dioxus::prelude::*;

/// Matches Bootstrap's `.container-lg` max-widths at each breakpoint.
pub const CONTAINER_LG_CLASS: &str =
    "mx-auto w-full max-w-[960px] px-4 lg:max-w-[960px] xl:max-w-[1140px] 2xl:max-w-[1320px]";

#[component]
pub fn Container(children: Element) -> Element {
    rsx! {
        div { class: CONTAINER_LG_CLASS, {children} }
    }
}

/// A full-bleed page-hero section with a background image, matching the
/// look of the old `*-wrap-non-bs`/`#*wrap` sections.
#[component]
pub fn HeaderWrap(
    bg_image: &'static str,
    min_height_class: &'static str,
    pt_class: &'static str,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "-mt-[70px] w-full bg-cover bg-center bg-no-repeat text-center text-white {min_height_class} {pt_class}",
            style: "background-image: url('{bg_image}')",
            Container { {children} }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { id: "copyrights",
            div { class: "w-full px-4",
                div { class: "credits flex flex-col items-center gap-2 md:flex-row",
                    div { class: "md:w-1/4", id: "social",
                        a { href: "https://www.linkedin.com/in/baxterjo",
                            i { class: "fa fa-linkedin" }
                        }
                    }

                    div { class: "md:w-1/2",
                        "Styling created with Instant template by "
                        a { href: "https://templatemag.com/", "TemplateMag" }
                    }
                    div { class: "md:w-1/4",


                        a {
                            id: "social",
                            // class:"flex-shrink-0",
                            href: "https://dioxuslabs.com/",
                            img {
                                src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                                alt: "Dioxus Labs Icon",
                            }
                            "Powered by Dioxus"
                        }

                    }


                }
            }
        }
    }
}
