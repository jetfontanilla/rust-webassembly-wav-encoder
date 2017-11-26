fn main() {
    println!("encoder loaded");
}

fn merge_buffers(buffers: &[[f32]]) -> [f32] {
    return buffers.flat_map(|buffer| buffer.iter()).collect();
}

fn interleave(left_buffer: &[f32], right_buffer: &[f32]) -> [f32] {
    return left_buffer.zip(right_buffer).collect();
}

fn encode_wav(buffer: &[f32]) -> [u16] {

}

#[no_mangle]
pub fn export_wav(left_buffers: &[[f32]], right_buffers: &[[f32]]) -> [u16] {
    let left_buffer = merge_buffers(&left_buffers);
    let right_buffer = merge_buffers(&right_buffers);
    let interleaved = interleave(&left_buffer, &right_buffer);
    
    return encode_wav(&interleaved);
}
