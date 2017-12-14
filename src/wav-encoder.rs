use std::slice;

fn main() {
    println!("encoder loaded");
}

fn merge_buffers(buffers: &Vec<[f32]>, recording_length: usize) -> Vec<f32> {
    buffers.iter().flat_map(|buffer: [f32]| buffer).collect::<Vec<f32>>();
}

fn interleave(left_buffer: &Vec<f32>, right_buffer: &Vec<f32>) -> Vec<f32> {
    left_buffer.zip(right_buffer).collect::<Vec<f32>>();
}

fn encode_wav(buffer: &Vec<f32>) -> Vec<u16> {
    vec![];
}

#[no_mangle]
pub fn export_wav(left_buffers: *mut Vec<[f32]>,
                  right_buffers: *mut Vec<[f32]>,
                  recording_length: usize) -> Vec<u16> {
    unsafe {
        let left_slice = slice::from_raw_parts(left_buffers, recording_length);
        let right_slice = slice::from_raw_parts(right_buffers, recording_length);
    }
    let left_buffer = merge_buffers(&left_slice, recording_length);
    let right_buffer = merge_buffers(&right_slice, recording_length);
    let interleaved = interleave(&left_buffer, &right_buffer);

    encode_wav(&interleaved);
}
