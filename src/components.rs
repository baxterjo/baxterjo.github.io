pub mod page_not_found;

use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            class: "navbar navbar-default navbar-fixed-top",
            div {
                class: "container",
                div {
                    class:"navbar-header",
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
