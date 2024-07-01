mod utils;

// or 
// mod utils {
//     pub fn set_panic_hook() {
//         // When the `console_error_panic_hook` feature is enabled, we can call the
//         // `set_panic_hook` function at least once during initialization, and then
//         // we will get better error messages if our code ever panics.
//         //
//         // For more details see
//         // https://github.com/rustwasm/console_error_panic_hook#readme
//         #[cfg(feature = "console_error_panic_hook")]
//         console_error_panic_hook::set_once();
//     }
// }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    utils::set_panic_hook();
    alert("Hello, my-app2!");
}
