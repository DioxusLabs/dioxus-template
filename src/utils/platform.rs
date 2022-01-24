//! `platform::startup` will check your target platform and use different launch.

use dioxus::prelude::*;

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub fn startup(root: Component) {

    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app); 
}

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub fn startup(root: Component) {
    dioxus::desktop::launch(root); 
}

pub fn is_wasm() -> bool {
    cfg!(any(target_arch = "wasm32", target_arch = "wasm64"))
}