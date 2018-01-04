use std::slice;
use std::cmp;

fn main() {
    println!("encoder loaded");
}

fn merge_buffers(buffers: &Vec<[f32]>, recording_length: usize) -> Vec<f32> {
    buffers.iter().flat_map(|buffer: [f32]| buffer).collect::<Vec<f32>>()
}

fn interleave(left_buffer: &Vec<f32>, right_buffer: &Vec<f32>) -> Vec<f32> {
    left_buffer.zip(right_buffer).collect::<Vec<f32>>()
}

fn encode_wav(buffer: &Vec<f32>, sample_rate: f16, mono: boolean) -> Vec<u16> {
    let mut stack: Vec<u16> = Vec::with_capacity(44 + buffer.len() * 2);

    /* RIFF identifier */
    stack.extend("RIFF".as_bytes());

    /* file length */
    let chunk_size = 32 + buffer.len() * 2;
    stack.extend(chunk_size.to_le());

    /* RIFF type */
    stack.extend("WAVE".as_bytes());

    /* format chunk identifier */
    stack.extend("fmt".as_bytes());

    /* format chunk length */
    stack.extend(16.to_le());

    /* sample format (raw) */
    stack.extend(1.to_le());

    /* channel count */
    if (mono) {
        stack.extend(1.to_le());
    } else {
        stack.extend(2.to_le());
    }

    /* sample rate */
    stack.extend(sampleRate.to_le());

    /* byte rate (sample rate * block align) */
    stack.extend((sampleRate * 4).to_le());

    /* block align (channel count * bytes per sample) */
    stack.extend(4.to_le());

    /* bits per sample */
    stack.extend(16.to_le());

    /* data chunk identifier */
    stack.extend("data".as_bytes());

    /* data chunk length */
    stack.extend((buffer.len() * 2).to_le());


    to_16_bit_pcm(&stack, buffer)
}

fn to_16_bit_pcm(stack: &Vec<u16>, buffer: &Vec<f32>) -> Vec<u16> {
    for (_, input) in buffer.iter().enumerate() {
        let s = cmp::max(-1, cmp::min(1, input));
        if (s < 0) {
            stack.extend((s * 0x8000).to_le());
        } else {
            stack.extend((s * 0x7FFF).to_le());
        }
    }

    stack
}


#[no_mangle]
pub fn export_wav(left_buffers: *mut Vec<[f32]>,
                  right_buffers: *mut Vec<[f32]>,
                  sample_rate: f16,
                  recording_length: usize) -> Vec<u16> {
    unsafe {
        let left_slice = slice::from_raw_parts(left_buffers, recording_length);
        let right_slice = slice::from_raw_parts(right_buffers, recording_length);
    }
    let left_buffer = merge_buffers(&left_slice, recording_length);
    let right_buffer = merge_buffers(&right_slice, recording_length);
    let interleaved = interleave(&left_buffer, &right_buffer);

    encode_wav(&interleaved, sample_rate, false)
}
