mod utils;

use wasm_bindgen::prelude::*;

use base64_rs::base64_encoder::Base64Encoder;
use base64_rs::base64_decoder::Base64Decoder;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, base64-wasm-rs!");
}

#[wasm_bindgen]
pub fn string_to_base64(data: &str) -> String {
    let enc = Base64Encoder::new(data.as_bytes());
    let encoded = enc.encode();
    let encoded = match encoded {
        Ok(res) => {
            res
        },
        Err(err) => {
            format!("an error occurred: {}", err)
        }
    };

    encoded
}

#[wasm_bindgen]
pub fn base64_to_string(data: &str) -> String {
    let dec = Base64Decoder::new(data);
    let decoded = dec.decode();
    let decoded = match decoded {
        Ok(res) => {
            res
        },
        Err(err) => {
            format!("an error occurred: {}", err)
        }
    };

    decoded
}
