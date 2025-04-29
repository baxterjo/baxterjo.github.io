// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use baxterjo_webapp::app;
use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // launch the web app
    launch(app::app);
}
