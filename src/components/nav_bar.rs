use crate::capitalize;
use crate::components::Container;
use crate::router::Route;
use dioxus::prelude::*;
use tracing::debug;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn NavBar() -> Element {
    let statics = Route::static_routes();
    debug!("STATICS: {:?}", statics);
    debug!("SITE_MAP: {:?}", Route::SITE_MAP);

    let mut menu_open = use_signal(|| false);
    let mut nav_hidden = use_signal(|| false);

    // Collapse nav bar when scrolling down and expand when scrolling up.
    use_effect(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let mut last_y = window.scroll_y().unwrap_or(0.0);

        let on_scroll = Closure::<dyn FnMut()>::new(move || {
            let Some(window) = web_sys::window() else {
                return;
            };
            let Ok(y) = window.scroll_y() else {
                return;
            };
            if y <= 10.0 {
                nav_hidden.set(false);
            } else if y > last_y {
                nav_hidden.set(true);
            } else if y < last_y {
                nav_hidden.set(false);
            }
            last_y = y;
        });

        let _ =
            window.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());
        // Leak the closure so it stays alive for the life of the listener (the nav bar never unmounts).
        on_scroll.forget();
    });

    rsx! {
        nav {
            class: if nav_hidden() && !menu_open() {
                "sticky top-0 z-50 -translate-y-full bg-black/50 transition-transform duration-300"
            } else {
                "sticky top-0 z-50 translate-y-0 bg-black/50 transition-transform duration-300"
            },
            Container {
                div { class: "flex flex-wrap items-center justify-between py-2",
                    div { class: "flex items-center",
                        button {
                            r#type: "button",
                            class: "mr-2 rounded border border-white/50 px-2 py-1 text-white lg:hidden",
                            onclick: move |_| menu_open.set(!menu_open()),
                            i { class: "fa fa-bars" }
                        }
                        Link {
                            class: "ml-2 text-lg font-bold text-[#1abc9c] hover:text-[#1abc9c]",
                            to: Route::Home {},
                            "JORDAN BAXTER"
                        }
                    }
                    ul {
                        class: if menu_open() {
                            "m-0 flex w-full list-none flex-col p-0 lg:ml-auto lg:w-auto lg:flex-row lg:items-center"
                        } else {
                            "m-0 hidden w-full list-none flex-col p-0 lg:ml-auto lg:flex lg:w-auto lg:flex-row lg:items-center"
                        },
                        li {
                            NavLink { route_to: Route::Home {} }
                        }
                        li {
                            NavLink { route_to: Route::About {} }
                        }
                        li {
                            NavLink { route_to: Route::Experience {} }
                        }
                        li {
                            NavLink { route_to: Route::ProjectsRoot {} }
                        }
                        li {
                            NavLink { route_to: Route::Contact {} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(route_to: Route) -> Element {
    let current_route: Route = use_route();
    let (route_name, link_class) = if route_to == (Route::Home {}) {
        let link_class = if current_route == (Route::Home {}) {
            "block py-2 text-[#1abc9c] lg:px-2"
        } else {
            "block py-2 text-white hover:text-[#1abc9c] lg:px-2"
        };
        ("Home".to_string(), link_class)
    } else {
        let route_str = route_to.to_string();
        let link_class = if current_route.to_string().contains(&route_str) {
            "block py-2 text-[#1abc9c] lg:px-2"
        } else {
            "block py-2 text-white hover:text-[#1abc9c] lg:px-2"
        };
        let route_name = capitalize(route_str.replace("/", "").as_str());
        (route_name, link_class)
    };

    rsx! {
        Link { class: link_class, to: route_to.clone(), {route_name} }
    }
}
