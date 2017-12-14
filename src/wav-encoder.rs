use std::slice;

fn main() {
    println!("encoder loaded");
}

fn merge_buffers(buffers: &[[f32]], recording_length: usize) -> [f32] {
    buffers.flat_map(|buffer| buffer.iter()).collect::<[f32]>();
}

fn interleave(left_buffer: &[f32], right_buffer: &[f32]) -> [f32] {
    left_buffer.zip(right_buffer).collect::<[f32]>();
}

fn encode_wav(buffer: &[f32]) -> [u16] {
    [];
}

#[no_mangle]
pub fn export_wav(left_buffers: *mut [[f32]],
                  right_buffers: *mut [[f32]],
                  recording_length: usize) -> [u16] {
    unsafe {
        let left_slice = slice::from_raw_parts(left_buffers, recording_length);
        let right_slice = slice::from_raw_parts(right_buffers, recording_length);
    }
    let left_buffer = merge_buffers(&left_slice, recording_length);
    let right_buffer = merge_buffers(&right_slice, recording_length);
    let interleaved = interleave(&left_buffer, &right_buffer);

    encode_wav(&interleaved);
}
