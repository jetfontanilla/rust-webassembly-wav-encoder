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
    if right_buffer.length() == 0 {
        // @TODO how to return original array instead of a clone
        return left_buffer.clone();
    }

    let left_size = left_buffer.length();
    let right_size = right_buffer.length();
    let combined_length = left_size + right_size;
    let interleaved_buffer = Float32Array::new_with_length(combined_length);

    let mut ctr = 0;
    let mut next_index = 0;
    while next_index < combined_length {
        let left_float = left_buffer.get_index(ctr);
        let right_float = right_buffer.get_index(ctr);
        if ctr < left_size {
            interleaved_buffer.set_index(next_index, left_float);
            next_index += 1
        }
        if ctr < right_size {
            interleaved_buffer.set_index(next_index, right_float);
            next_index += 1
        }
        ctr += 1
    }
    interleaved_buffer
}

fn encode_wav(buffer: &Float32Array, first_buffer: bool) -> Uint16Array {
    let encoded = Uint16Array::new_with_length(buffer.length());

    encoded
}

#[wasm_bindgen]
pub fn export_wav(left_buffer: &Float32Array, right_buffer: &Float32Array, first_buffer: bool) -> Uint16Array {
    let interleaved = interleave(left_buffer, right_buffer);    
    let encoded = encode_wav(&interleaved, first_buffer);

    return encoded;
}