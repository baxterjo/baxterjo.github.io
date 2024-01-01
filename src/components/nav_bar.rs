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
                class: "container",
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
                        class: "navbar-brand",
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
                            NavLink { route_to: Route::Projects {  }}
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
        Outlet::<Route> {}
    }
}

#[component]
fn NavLink(cx: Scope, route_to: Route) -> Element {
    let current_route: Route = use_route(&cx).unwrap();
    let route_name = if *route_to == (Route::Home {}) {
        "Home".to_string()
    } else {
        capitalize(route_to.to_string().replace("/", "").as_str())
    };
    let link_class = if *route_to == current_route {
        "nav-link active"
    } else {
        "nav-link"
    };
    render! {
        Link {
            class: link_class,
            to: route_to.clone(),
            route_name,
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
