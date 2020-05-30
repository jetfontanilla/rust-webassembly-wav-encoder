mod utils;

use wasm_bindgen::prelude::*;
use js_sys::ArrayBuffer;
use js_sys::DataView;
use js_sys::Float32Array;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn interleave(left_buffer: Float32Array, right_buffer: Float32Array) -> Float32Array {
    if right_buffer.length() == 0 {
        return left_buffer;
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

fn encode_wav(interleaved_buffer: &Float32Array, sample_rate: u32, first_buffer: bool) -> DataView {
    const WAV_HEADER_LENGTH:u32 = 44;
    let buffer_length:u32 = interleaved_buffer.length();
    let mut current_offset:u32 = 0;
    if first_buffer {
        current_offset = WAV_HEADER_LENGTH;
    }
    let wav_buffer = ArrayBuffer::new(buffer_length);
    let wav_data_view = DataView::new(&wav_buffer, 0, (buffer_length + current_offset) as usize);

    if first_buffer {
        set_wav_header(&wav_data_view, sample_rate, buffer_length);
    }

    // write the PCM samples converting from f32 to u16
    for i in 0..buffer_length {
        let current_value:f32 = interleaved_buffer.get_index(i);
        let sample:f32 = if current_value > 1.0 {
            1.0
        } else if current_value < -1.0 {
            -1.0
        } else {
            current_value
        };

        if sample < 0.0 {
            wav_data_view.set_uint16_endian(current_offset as usize, (sample * 0x8000 as f32) as u16, true);
        } else {
            wav_data_view.set_uint16_endian(current_offset as usize, (sample * 0x7FFF as f32) as u16, true);
        }

        current_offset += 2;
    }

    wav_data_view
}

fn set_wav_header(wav_data_view: &DataView, sample_rate: u32, buffer_length: u32) {
    // write the WAV container, check spec at: https://ccrma.stanford.edu/courses/422/projects/WaveFormat/
    write_to_dataview(wav_data_view, 0, "RIFF");
    wav_data_view.set_uint32_endian(4, 44 + buffer_length * 2, true);
    write_to_dataview(wav_data_view, 8, "WAVE");
    write_to_dataview(wav_data_view, 12, "fmt ");
    wav_data_view.set_uint32_endian(16, 16, true);
    wav_data_view.set_uint16_endian(20, 1, true);
    wav_data_view.set_uint16_endian(22, 1, true);
    wav_data_view.set_uint32_endian(24, sample_rate, true);
    wav_data_view.set_uint32_endian(28, sample_rate * 4, true);
    wav_data_view.set_uint16_endian(32, 4, true);
    wav_data_view.set_uint16_endian(34, 16, true);
    write_to_dataview(wav_data_view, 36, "data");
    wav_data_view.set_uint32_endian(40, buffer_length * 2, true);
}

fn write_to_dataview(view: &DataView, offset: usize, string: &str) {
    let bytes = string.as_bytes();
    for i in 0..bytes.len() {
        view.set_uint8(offset + i, bytes[i]);
    }
}

#[wasm_bindgen]
pub fn export_wav(
    left_buffer: Float32Array, 
    right_buffer: Float32Array,
    sample_rate: u32,
    first_buffer: bool
) -> DataView {
    let interleaved = interleave(left_buffer, right_buffer);    
    return encode_wav(&interleaved, sample_rate, first_buffer);
}