pub mod page_not_found;

use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            class: "navbar",
            li {
                Link {
                    // The Link component will navigate to the route specified
                    // in the target prop which is checked to exist at compile time
                    to: Route::Home {},
                    "Home"
                }
            }
            li {
                Link {
                    // The Link component will navigate to the route specified
                    // in the target prop which is checked to exist at compile time
                    to: Route::PageNotFound {route: vec!["About".to_string()]},
                    "About"
                }
            }

        }
        Outlet::<Route> {}
    }
}
