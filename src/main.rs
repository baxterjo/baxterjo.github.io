// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use baxterjo_webapp::app;
use dioxus::prelude::*;

fn main() {
    tracing_wasm::set_as_global_default();
    // launch the web app
    launch(app::app);
}
