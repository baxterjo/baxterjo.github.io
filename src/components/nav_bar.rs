use crate::capitalize;
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::debug;

#[component]
pub fn NavBar(cx: Scope) -> Element {
    let statics = Route::static_routes();
    debug!("STATICS: {:?}", statics);
    debug!("SITE_MAP: {:?}", Route::SITE_MAP);
    // let about_route = Route::from_str("/about");
    // debug!("From Str: {:?}", about_route);
    render! {
        nav {
            class: "navbar navbar-expand-lg navbar-dark bg-dark navbar-fixed-top",
            div {
                class: "container-lg",
                div {
                    class:"navbar-header",
                    button {
                        r#type: "button",
                        class: "navbar-toggler",
                        "data-bs-toggle":"collapse",
                        "data-bs-target":".navbar-collapse",
                        span {
                            class: "navbar-toggler-icon"
                        }
                    }
                    Link {
                        class: "navbar-brand ms-2",
                        to: Route::Home {},
                        "JORDAN BAXTER"
                    }

                }
                div {
                    class: "navbar-collapse collapse",
                    ul {
                        class: "navbar-nav ms-auto",
                        li {
                            class: "nav-item",
                            NavLink{ route_to: Route::Home{}}
                        }
                        li {
                            class: "nav-item",
                            NavLink { route_to: Route::About {  }}
                        }
                        li {
                            class: "nav-item",
                            NavLink { route_to: Route::Experience {  }}
                        }
                        li {
                            class: "nav-item",
                            NavLink { route_to: Route::ProjectsRoot {  }}
                        }
                        li {
                            class: "nav-item",
                            NavLink { route_to: Route::Education {  }}
                        }
                        li {
                            class: "nav-item",
                            NavLink { route_to: Route::Contact {  }}
                        }
                    }
                }
            }



        }
        // debug!("{:#?}", Route::SITE_MAP)

    }
}

#[component]
fn NavLink(cx: Scope, route_to: Route) -> Element {
    let current_route: Route = use_route(&cx).unwrap();
    let (route_name, link_class) = if *route_to == (Route::Home {}) {
        let link_class = if current_route == (Route::Home {}) {
            "nav-link active"
        } else {
            "nav-link"
        };
        ("Home".to_string(), link_class)
    } else {
        let route_str = route_to.to_string();
        let link_class = if current_route.to_string().contains(&route_str) {
            "nav-link active"
        } else {
            "nav-link"
        };
        let route_name = capitalize(route_str.replace("/", "").as_str());
        (route_name, link_class)
    };

    render! {
        Link {
            class: link_class,
            to: route_to.clone(),
            route_name,
        }
    }
}
