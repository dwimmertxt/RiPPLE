pub fn encode(_id: &str, data_frames: &Vec<Vec<u32>>) -> Vec<u8> {
    let num_channels:       u16 = 1;
    let sample_rate:        u32 = data_frames[0].len() as u32;
    let bits_per_sample:    u16  = 32;
    let subchunk_2_size:    u32 = sample_rate * (data_frames.len() as u32) * 4;
    
    let mut wav_bytes = Vec::new();
    wav_bytes.extend(riff_header(subchunk_2_size));
    wav_bytes.extend(fmt_subchunk(sample_rate, num_channels, bits_per_sample));
    wav_bytes.extend(data_subchunk(subchunk_2_size));

    for frame in data_frames {
        for datum in frame {
            wav_bytes.extend(datum.to_le_bytes());
        }
    }
     
    wav_bytes
}


fn riff_header(subchunk_2_size: u32) -> Vec<u8> {
    let mut riff_header = Vec::new();
    riff_header.extend(vec![82u8, 73, 70, 70]);               // ChunkID: "RIFF"
    riff_header.extend((36 + subchunk_2_size).to_le_bytes()); // ChunkSize: 36 + Subchunk2Size
    riff_header.extend(vec![87u8, 65, 86, 69]);               // Format: "WAVE"
    riff_header
}


fn fmt_subchunk(sample_rate: u32, num_channels: u16, bits_per_sample: u16) -> Vec<u8> {
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


fn data_subchunk(subchunk_2_size: u32) -> Vec<u8> {
    let mut data_subchunk = Vec::new();
    data_subchunk.extend(vec![100u8, 97, 116, 97]); // Subchunk2ID: "data"
    data_subchunk.extend(subchunk_2_size.to_le_bytes());
        // Subchunk2Size: NumSamples * NumChannels * BitsPerSample/8
    data_subchunk
}
