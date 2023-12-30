pub mod page_not_found;

use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            class: "border-transparent bg-black top-0 border-b-1 border-x-0 border-t-0 rounded-none fixed inset-x-0 z-1030  min-h-50 mb-20",
            div {
                class: "container",
                div {
                    class:"mx-0 float-left",
                    li {
                        class: "navbar-brand",
                        Link {
                            to: Route::Home {},
                            "JORDAN BAXTER"
                        }
                    }
                }
            }
            div {
                class: "navbar-collapse collapse",
                ul {
                    class: "nav navbar-nav navbar-right",
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
                    li {
                        Link {
                            to: Route::PageNotFound { route: vec!["Experience".to_string()] },
                            "Experience"
                        }
                    }
                    li {
                        Link {
                            to: Route::PageNotFound { route: vec!["Projects".to_string()] },
                            "Projects"
                        }
                    }
                    li {
                        Link {
                            to: Route::PageNotFound { route: vec!["Education".to_string()] },
                            "Education"
                        }
                    }
                    li {
                        Link {
                            to: Route::PageNotFound { route: vec!["Contact".to_string()] },
                            "Contact"
                        }
                    }
                }
            }


        }
        Outlet::<Route> {}
    }
}
