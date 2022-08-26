
pub fn ripl(bytes: Vec<u8>) -> (&'static str, Vec<Vec<u32>>) {   
    match std::str::from_utf8(&bytes[4..8]).unwrap() {
            "TIME" => ("TIME", time(bytes)),
            "FREQ" => ("FREQ", frequency(bytes)),
            "HARM" => ("HARM", harmonic(bytes)),
            _      => ("VOID", Vec::new()),
        }
}


fn time(bytes: Vec<u8>) -> Vec<Vec<u32>> {
    let width         = u32::from_le_bytes(*&bytes[8..12].try_into().unwrap());
    let _height        = u32::from_le_bytes(*&bytes[12..16].try_into().unwrap());
    let _depth         = u32::from_le_bytes(*&bytes[16..20].try_into().unwrap());
    let _resolution    = u32::from_le_bytes(*&bytes[20..24].try_into().unwrap());
    let bytes_per_val = bytes[36] as u32;

    let vec_max = (width * bytes_per_val) as usize;
    let delim = ((width * bytes_per_val) - bytes_per_val) as usize;

    let mut time_f: Vec<Vec<u32>> = Vec::new();
    let mut time_d: Vec<u32> = Vec::new();
    let data = &bytes[37..];
    for (idx, val) in data.iter().enumerate().step_by(bytes_per_val as usize) {     
        let amplitude_bytes: [u8; 4] = [*val, data[idx+1], data[idx+2], 0u8];
        let amplitude = u32::from_le_bytes(amplitude_bytes);

        time_d.push(amplitude);
        
        if idx.rem_euclid(vec_max) == delim {
            time_f.push(time_d.clone());
            time_d = Vec::new();
        }
    }
    time_f
}


fn frequency(bytes: Vec<u8>) -> Vec<Vec<u32>> {
    let num_f         = u32::from_le_bytes(*&bytes[8..12].try_into().unwrap());
    let bytes_per_val = bytes[36] as u32;

    let vec_max = (num_f * bytes_per_val) as usize;
    let delim = ((num_f * bytes_per_val) - bytes_per_val) as usize;

    let mut freq_f: Vec<Vec<u32>> = Vec::new();
    let mut freq_d: Vec<u32> = Vec::new();
    let data = &bytes[37..];
    for (idx, val) in data.iter().enumerate().step_by(bytes_per_val as usize) {     
        let magnitude_bytes: [u8; 4] = [*val, data[idx+1], data[idx+2], data[idx+3]];
        let magnitude = u32::from_le_bytes(magnitude_bytes);

        freq_d.push(magnitude);
        if idx.rem_euclid(vec_max) == delim {
            freq_f.push(freq_d.clone());
            freq_d = Vec::new();
        }
    }
    freq_f
}


fn harmonic(_bytes: Vec<u8>) -> Vec<Vec<u32>> {
    let harm_f: Vec<Vec<u32>> = Vec::new();
    harm_f
}