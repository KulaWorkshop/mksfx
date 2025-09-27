use crate::libpsxav;
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::error::Error;
use std::path::Path;

fn clamp_i16(v: i32) -> i16 {
    if v > i16::MAX as i32 {
        i16::MAX
    } else if v < i16::MIN as i32 {
        i16::MIN
    } else {
        v as i16
    }
}

fn decode_block_int(block: &[u8], hist: (i32, i32)) -> (Vec<i16>, (i32, i32)) {
    // integer coefficient tables (spec * 64)
    const K0: [i32; 5] = [0, 60, 115, 98, 122];
    const K1: [i32; 5] = [0, 0, -52, -55, -60];

    if block.len() < 16 {
        return (Vec::new(), hist);
    }

    let shift = (block[0] & 0x0F) as i32;
    let mut filter = ((block[0] >> 4) & 0x0F) as usize;
    if filter > 4 {
        filter = 0;
    }
    let k0 = K0[filter];
    let k1 = K1[filter];

    let mut samples: Vec<i16> = Vec::with_capacity(28);
    let mut hist1 = hist.0;
    let mut hist2 = hist.1;

    // 14 data bytes contain 28 nibbles -> 28 samples
    for i in 0..14 {
        let byte = block[2 + i];
        // low nibble first (shift 0), then high nibble (shift 4)
        for nib_shift in [0, 4].iter() {
            let mut nib = ((byte >> nib_shift) & 0x0F) as i32;
            // sign extend 4-bit to signed
            if nib >= 8 {
                nib -= 16;
            }

            // scale nibble: (nibble << 12) >> shift
            let mut sample = (nib << 12) >> shift;

            // predictor: + ((hist1 * k0) + (hist2 * k1) + 32) >> 6
            sample += ((hist1 * k0) + (hist2 * k1) + 32) >> 6;

            // clamp to 16-bit
            let s_clamped = clamp_i16(sample);
            samples.push(s_clamped);

            hist2 = hist1;
            hist1 = s_clamped as i32;
        }
    }

    (samples, (hist1, hist2))
}

pub fn decode_to_wav<P: AsRef<Path>>(
    path: P,
    data: &[u8],
    sample_rate: u32,
) -> Result<(), Box<dyn Error>> {
    let mut samples: Vec<i16> = Vec::new();
    let mut hist = (0i32, 0i32);

    let mut pos = 0usize;
    while pos + 16 <= data.len() {
        let block = &data[pos..pos + 16];
        let (block_samples, new_hist) = decode_block_int(block, hist);
        samples.extend(block_samples);
        hist = new_hist;
        pos += 16;
    }

    // create wav spec
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    // write samples
    let mut writer = WavWriter::create(path, spec)?;
    for &sample in &samples {
        writer.write_sample(sample)?;
    }

    // finalize
    writer.finalize()?;

    Ok(())
}

pub fn encode_from_wav(path: &Path, loop_enabled: bool) -> Result<Vec<u8>, Box<dyn Error>> {
    // open wav file
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();

    // validate wav
    if spec.channels != 1 {
        return Err("WAV file must have only 1 track (mono).".into());
    }

    if spec.bits_per_sample != 16 {
        return Err("WAV file must be 16-bit.".into());
    }

    // encode samples
    let samples: Vec<i16> = reader.samples::<i16>().filter_map(Result::ok).collect();
    let loop_start = if loop_enabled { 0 } else { -1 };
    let buffer = libpsxav::spu_encode(samples, loop_start);
    Ok(buffer)
}

pub fn detect_loop(buffer: &[u8]) -> bool {
    let flags1 = buffer.get(1).copied().unwrap_or(0u8);
    let flags2 = buffer.get(17).copied().unwrap_or(0u8);
    (flags1 & 0x04 != 0) || (flags2 & 0x04 != 0)
}
