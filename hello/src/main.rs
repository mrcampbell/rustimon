use dioxus::prelude::*;
use log::{LevelFilter, info};
fn main() {
    // init debug tool for WebAssembly
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    dioxus::web::launch(app); 
}

fn app(cx: Scope) -> Element {

log::info!("Some info");
log::error!("Error message");

    cx.render(rsx! (
        div {
            style: "text-align: center;",
            button {
              class: "btn btn-primary",
                "Fuck!"
            }
            h1 { class: "bg-red-500", "🌗 Dioxus 🚀!" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
    ))
}