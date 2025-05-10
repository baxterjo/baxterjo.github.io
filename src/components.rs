pub mod gallery;
pub mod layout;
pub mod nav_bar;
pub mod page_not_found;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { id: "copyrights",
            div { class: "container-fluid",
                div { class: "credits row align-items-center",
                    div { class: "col-md-3 col-sm-12", id: "social",
                        a { href: "https://www.linkedin.com/in/baxterjo",
                            i { class: "fa fa-linkedin" }
                        }
                    }

                    div { class: "col-md-6 col-sm-12",
                        "Styling created with Instant template by "
                        a { href: "https://templatemag.com/", "TemplateMag" }
                    }
                    div { class: "col-md-3 col-sm-12",


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
