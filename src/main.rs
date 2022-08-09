#![feature(exclusive_range_pattern)]

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};


fn main() {
    // 1. generate wave header
    // 2. generate audio data
    // 3. write header & audio data into file
    create_wave();
}


fn create_wave() {
    let duration = 10;
    let num_channels = 2;
    let sample_rate = 44100;
    let bits_per_sample = 24;

    let audio_data = generate_audio_data(duration);
    let num_samples = audio_data.len() as u32;

    let mut wave_buffer = Vec::new();
    wave_buffer.extend(generate_riff_header(num_samples, num_channels, bits_per_sample));
    wave_buffer.extend(generate_fmt_subchunk(sample_rate, num_channels, bits_per_sample));
    wave_buffer.extend(generate_data_subchunk(num_samples, num_channels, bits_per_sample));
    wave_buffer.extend(audio_data);

    let mut f = File::create("ripple.wav").expect("unable to write file");
    f.write_all(&wave_buffer).expect("Could not write");
}


fn generate_audio_data(duration: i32) -> Vec<u8> {
    let num_samples = duration * 44100;
    let mut audio_data = Vec::new();
    for t in 0..num_samples {
        let rgb = hsl_to_rgb(
            (conv_base(t, 360) % 360) as f64,
            ((conv_base(t + 100, 100) % 100) as f64) * 0.01,
            ((conv_base(t + 50, 100) % 100) as f64) * 0.01,
        );
        let rgb_decimal = rgb_to_decimal(rgb[0], rgb[1], rgb[2]);
        let rgb_bytes = rgb_decimal.to_le_bytes();
        for i in 0..3 {
            audio_data.push(rgb_bytes[i]);
        }
    }
    audio_data
}


fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
    // h [0.0, 360.0], s [0.0, 1.0], l [0.0, 1.0]
    let c = (1.0 - f64::abs((2.0 * l) - 1.0)) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - f64::abs((h_prime % 2.0) - 1.0));
    let m = l - (c / 2.0);
    let rgb = match h_prime as i32 {
        0..1 => [c + m, x + m, m],
        1..2 => [x + m, c + m, m],
        2..3 => [m, c + m, x + m],
        3..4 => [m, x + m, c + m],
        4..5 => [x + m, m, c + m],
        _ => [c + m, m, x + m],
    };
    [
        (rgb[0] * 255.0) as u8,
        (rgb[1] * 255.0) as u8,
        (rgb[2] * 255.0) as u8,
    ]
}


fn conv_base(mut n: i32, b: i32) -> i32 {
    // converts number n into base b
    // then returns converted num modulo b
    let mut digits = vec![0];
    while n > 0 {
        digits.push(n % b);
        n = n / b;
    }
    digits.iter().sum()
}


fn rgb_to_decimal(r: u8, g: u8, b: u8) -> i32 {
    (65536 * r as i32) + (256 * g as i32) + b as i32
}


fn generate_riff_header(num_samples: u32, num_channels: u16, bits_per_sample: u16) -> Vec<u8> {
    let mut riff_header = Vec::new();
    riff_header.extend(vec![82u8, 73, 70, 70]); // ChunkID: "RIFF"
    riff_header
        .extend((36u32 + ((num_samples) * (num_channels as u32) * (bits_per_sample as u32) / 8)).to_le_bytes());
        // ChunkSize: 36 + Subchunk2Size
    riff_header.extend(vec![87u8, 65, 86, 69]); // Format: "WAVE"
    riff_header
}


fn generate_fmt_subchunk(sample_rate: u32, num_channels: u16, bits_per_sample: u16) -> Vec<u8> {
    let mut fmt_subchunk = Vec::new();
    fmt_subchunk.extend(vec![102u8, 109, 116, 32]); // Subchunk1ID: "fmt "
    fmt_subchunk.extend(vec![16u8, 0, 0, 0]); // Subchunk1Size: 16 = PCM
    fmt_subchunk.extend(vec![1u8, 0]); // AudioFormat: PCM = 1
    fmt_subchunk.extend(num_channels.to_le_bytes()); // NumChannels: Mono = 1
    fmt_subchunk.extend(sample_rate.to_le_bytes()); // SampleRate: 44100
    fmt_subchunk
        .extend((sample_rate * (num_channels as u32) * (bits_per_sample as u32) / 8).to_le_bytes());
        // ByteRate: SampleRate * NumChannels * BitsPerSample/8
    fmt_subchunk
        .extend((num_channels * bits_per_sample / 8).to_le_bytes());
        // BlockAlign: NumChannels * BitsPerSample/8
    fmt_subchunk.extend(bits_per_sample.to_le_bytes()); // BitsPerSample
    fmt_subchunk
}


fn generate_data_subchunk(num_samples: u32, num_channels: u16, bits_per_sample: u16) -> Vec<u8> {
    let mut data_subchunk = Vec::new();
    data_subchunk.extend(vec![100u8, 97, 116, 97]); // Subchunk2ID: "data"
    data_subchunk
        .extend((num_samples * (num_channels as u32) * (bits_per_sample as u32) / 8).to_le_bytes());
        // Subchunk2Size: NumSamples * NumChannels * BitsPerSample/8
    data_subchunk
}