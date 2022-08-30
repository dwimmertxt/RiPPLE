
pub fn ripl(bytes: Vec<u8>) -> (&'static str, Vec<Vec<u32>>) {   
    match std::str::from_utf8(&bytes[4..8]).unwrap() {
            "TIME" => ("TIME", time(bytes)),
            "FREQ" => ("FREQ", frequency(bytes)),
            "HARM" => ("HARM", harmonic(bytes)),
            _      => ("VOID", Vec::new()),
        }
}


fn time(bytes: Vec<u8>) -> Vec<Vec<u32>> {
    let width           = (u32::from_le_bytes(*&bytes[8..12].try_into().unwrap())) as usize;
    let _dimensionality = bytes[12] as u8;
    let _frames         = u32::from_le_bytes(*&bytes[13..17].try_into().unwrap());
    let bytes_per_datum = bytes[36] as usize;

    let frame_len = width * bytes_per_datum;
    let delim     = frame_len - bytes_per_datum;

    let mut time_frames: Vec<Vec<u32>> = Vec::new();
    let mut time_domain: Vec<u32> = Vec::new();
    let data = &bytes[37..];
    for (idx, byte) in data.iter().enumerate().step_by(bytes_per_datum) {     
        let amplitude_bytes: [u8; 4] = [*byte, data[idx+1], data[idx+2], 0u8];
        let amplitude = u32::from_le_bytes(amplitude_bytes);

        time_domain.push(amplitude);
        
        if idx.rem_euclid(frame_len) == delim {
            time_frames.push(time_domain.clone());
            time_domain = Vec::new();
        }
    }
    time_frames
}


fn frequency(bytes: Vec<u8>) -> Vec<Vec<u32>> {
    let data_per_frame  = (u32::from_le_bytes(*&bytes[8..12].try_into().unwrap())) as usize;
    let bytes_per_datum = bytes[36] as usize;

    let frame_len = data_per_frame * bytes_per_datum;
    let delim     = frame_len - bytes_per_datum;

    let mut freq_frames: Vec<Vec<u32>> = Vec::new();
    let mut freq_domain: Vec<u32> = Vec::new();
    let data = &bytes[37..];
    for (idx, byte) in data.iter().enumerate().step_by(bytes_per_datum) {     
        let magnitude_bytes: [u8; 4] = [*byte, data[idx+1], data[idx+2], data[idx+3]];
        let magnitude = u32::from_le_bytes(magnitude_bytes);

        freq_domain.push(magnitude);
        if idx.rem_euclid(frame_len) == delim {
            freq_frames.push(freq_domain.clone());
            freq_domain = Vec::new();
        }
    }
    freq_frames
}


fn harmonic(_bytes: Vec<u8>) -> Vec<Vec<u32>> {
    let harm_frames: Vec<Vec<u32>> = Vec::new();
    harm_frames
}