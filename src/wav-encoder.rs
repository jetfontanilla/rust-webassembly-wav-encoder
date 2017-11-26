fn main() {
    println!("encoder loaded");
}

fn merge_buffers(&buffers: [[f32]], recording_length: i16) -> [f32] {

}

fn interleave(&left_buffer: [f32], &right_buffer: [f32]) -> [f32] {

}

fn encode_wav(buffer: [f32]) -> [u16] {

}

#[no_mangle]
pub fn export_wav(left_buffers: [[f32]], right_buffers: [[f32]], recording_length: i16) -> [u16] {
    let left_buffer = merge_buffers(&left_buffers, recording_length);
    let right_buffer = merge_buffers(&right_buffers, recording_length);
    let interleaved = interleave(&left_buffer, &right_buffer);
    
    return encode_wav(interleaved);
}
