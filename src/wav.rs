use std::fs::File;
use std::io::{Write, Error};

use crate::maff::normalise;


pub fn generate(waveform_samples: &Vec<i32>) {
    let num_channels:       i32 = 1;
    let bits_per_sample:    i32 = 16;
    let sample_rate:        i32 = 44100;
    
    let mut waveform_samples_bytes = Vec::new();
    for sample in waveform_samples {
        waveform_samples_bytes.extend(
            (normalise(*sample, &vec![0, 22050]) as i16)
            .to_le_bytes());
    }
     
    let subchunk_2_size:    i32 = waveform_samples_bytes.len() as i32;

    let mut wav_buffer = Vec::new();
    wav_buffer.extend(riff_header(subchunk_2_size, num_channels, bits_per_sample));
    wav_buffer.extend(fmt_subchunk(sample_rate, num_channels, bits_per_sample));
    wav_buffer.extend(data_subchunk(subchunk_2_size, num_channels, bits_per_sample));
    wav_buffer.extend(waveform_samples_bytes);

    let wav_file = create_wav_file(&wav_buffer);
    match wav_file {
        Ok(()) => eprintln!("Successfully created ripple.wav"),
        Err(_) => eprintln!("Error creating ripple.wav"),
    }
}


fn riff_header(num_samples: i32, num_channels: i32, bits_per_sample: i32) -> Vec<u8> {
    let mut riff_header = Vec::new();
    riff_header.extend(vec![82u8, 73, 70, 70]); // ChunkID: "RIFF"
    riff_header
        .extend((36i32 + (num_samples * num_channels * bits_per_sample / 8)).to_le_bytes());
        // ChunkSize: 36 + Subchunk2Size
    riff_header.extend(vec![87u8, 65, 86, 69]); // Format: "WAVE"
    riff_header
}


fn fmt_subchunk(sample_rate: i32, num_channels: i32, bits_per_sample: i32) -> Vec<u8> {
    let mut fmt_subchunk = Vec::new();
    fmt_subchunk.extend(vec![102u8, 109, 116, 32]); // Subchunk1ID: "fmt "
    fmt_subchunk.extend(vec![16u8, 0, 0, 0]); // Subchunk1Size: 16 = PCM
    fmt_subchunk.extend(vec![1u8, 0]); // AudioFormat: PCM = 1
    fmt_subchunk.extend((num_channels as u16).to_le_bytes()); // NumChannels: Mono = 1
    fmt_subchunk.extend(sample_rate.to_le_bytes()); // SampleRate: 44100
    fmt_subchunk
        .extend((sample_rate * num_channels * bits_per_sample / 8).to_le_bytes());
        // ByteRate: SampleRate * NumChannels * BitsPerSample/8
    fmt_subchunk
        .extend(((num_channels * bits_per_sample / 8) as u16).to_le_bytes());
        // BlockAlign: NumChannels * BitsPerSample/8
    fmt_subchunk.extend((bits_per_sample as u16).to_le_bytes()); // BitsPerSample
    fmt_subchunk
}


fn data_subchunk(num_samples: i32, num_channels: i32, bits_per_sample: i32) -> Vec<u8> {
    let mut data_subchunk = Vec::new();
    data_subchunk.extend(vec![100u8, 97, 116, 97]); // Subchunk2ID: "data"
    data_subchunk
        .extend((num_samples * num_channels * bits_per_sample / 8).to_le_bytes());
        // Subchunk2Size: NumSamples * NumChannels * BitsPerSample/8
    data_subchunk
}


fn create_wav_file(data: &Vec<u8>) -> Result<(), Error> {
    let mut f = File::create("ripple.wav").expect("Unable to write file");
    f.write_all(&*data).expect("Could not write");
    Ok(())
}