fn main() {
    println!("encoder loaded");
}

#[no_mangle]
pub fn export_wav(left_buffer: f32, right_buffer: f32) -> i16 {
    return 1i16;
}
