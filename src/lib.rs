mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Boolean;
use js_sys::Float32Array;
use js_sys::Uint16Array;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn interleave(left_buffer: &Float32Array, right_buffer: &Float32Array) -> Float32Array {
    let combined_length = left_buffer.length() + right_buffer.length();
    let interleaved_buffer = Float32Array::new_with_length(combined_length);
    interleaved_buffer
    //left_buffer.zip(right_buffer).collect();
}

fn encode_wav(buffer: &Float32Array, first_buffer: Boolean) -> Uint16Array {
    let encoded = Uint16Array::new_with_length(buffer.length());

    encoded
}

#[wasm_bindgen]
pub fn export_wav(left_buffer: &Float32Array, right_buffer: &Float32Array, first_buffer: Boolean) -> Uint16Array {
    let interleaved = interleave(left_buffer, right_buffer);    
    let encoded = encode_wav(&interleaved, first_buffer);

    return encoded;
}