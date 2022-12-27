use dioxus::prelude::*;
use rustimon;

fn main() {
    // init debug tool for WebAssembly
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    dioxus::web::launch(app); 
}

fn app(cx: Scope) -> Element {
let x = rustimon::core::types::pokemon::StatGroup {hp: 1, atk: 2, def: 3, spec_atk: 4, spec_def: 5, spd: 6};
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            button {
              class: "btn btn-warning",
              "{x.atk}"
            }
            h1 { class: "bg-red-500", "🌗 Dioxus 🚀!" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
    ))
}