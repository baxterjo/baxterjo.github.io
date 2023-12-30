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
                    button {
                        r#type: "button",
                        class: "navbar-toggle",
                        "data-toggle":"collapse",
                        "data-target":".navbar-collapse"
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
                        class: "nav navbar-nav navbar-right",
                        li {
                            Link {
                                class:"smoothscroll",
                                // The Link component will navigate to the route specified
                                // in the target prop which is checked to exist at compile time
                                to: Route::Home {},
                                "Home"
                            }
                        }
                        li {
                            Link {
                                class:"smoothscroll",
                                // The Link component will navigate to the route specified
                                // in the target prop which is checked to exist at compile time
                                to: Route::PageNotFound {route: vec!["About".to_string()]},
                                "About"
                            }
                        }
                        li {
                            Link {
                                class:"smoothscroll",
                                to: Route::PageNotFound { route: vec!["Experience".to_string()] },
                                "Experience"
                            }
                        }
                        li {
                            Link {
                                class:"smoothscroll",
                                to: Route::PageNotFound { route: vec!["Projects".to_string()] },
                                "Projects"
                            }
                        }
                        li {
                            Link {
                                class:"smoothscroll",
                                to: Route::PageNotFound { route: vec!["Education".to_string()] },
                                "Education"
                            }
                        }
                        li {
                            Link {
                                class:"smoothscroll",
                                to: Route::PageNotFound { route: vec!["Contact".to_string()] },
                                "Contact"
                            }
                        }
                    }
                }
            }



        }
        Outlet::<Route> {}
    }
}
