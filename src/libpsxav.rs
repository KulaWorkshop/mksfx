use libc::c_int;

#[link(name = "libpsxav")]
unsafe extern "C" {
    fn psx_audio_spu_encode_simple(
        samples: *const i16,
        sample_count: c_int,
        output: *mut u8,
        loop_start: c_int,
    ) -> c_int;
}

pub fn spu_encode(samples: Vec<i16>, loop_start: i8) -> Vec<u8> {
    // sample count
    let sample_count = samples.len();

    // calculate output size
    const SAMPLES_PER_BLOCK: f64 = 28.0;
    const BYTES_PER_BLOCK: usize = 16;
    let output_size = (((sample_count as f64 + SAMPLES_PER_BLOCK - 1.0) / SAMPLES_PER_BLOCK).ceil()
        as usize)
        * BYTES_PER_BLOCK;

    // encode
    let mut output_buffer = vec![0u8; output_size];
    let encoded_length: libc::c_int = unsafe {
        psx_audio_spu_encode_simple(
            samples.as_ptr(),
            sample_count as c_int,
            output_buffer.as_mut_ptr(),
            loop_start as libc::c_int,
        )
    };

    // truncate buffer
    output_buffer.truncate(encoded_length as usize);
    output_buffer
}
