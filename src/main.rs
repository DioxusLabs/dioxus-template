mod utils;

use dioxus::prelude::*;

fn main() {
    // this function will check wich `target` you use
    utils::platform::startup(app);
}

fn app(cx: Scope) -> Element {

    let platform = if utils::platform::is_wasm() { "WebAssembly" } else { "Desktop" };

    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
            ul {
                li { "runtime platform: {platform}" }
            }
        }
    ))
}