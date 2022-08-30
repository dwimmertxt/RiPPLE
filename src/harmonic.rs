pub fn process(freq_frames: &Vec<Vec<u32>>) -> Vec<Vec<u32>> { 
    // process a number of frequency frames to obtain their respective harmonic series'
    let mut harm_frames: Vec<Vec<u32>> = Vec::new(); // harmonic frames; frame = harmonic series
    for freq_domain in freq_frames {
        harm_frames.push(harmonic_series(&freq_domain));
    }
    harm_frames
}


fn harmonic_series(freq_domain: &Vec<u32>) -> Vec<u32> {
    let mut harm_s: Vec<u32> = Vec::new();
    harm_s.extend(&fundamental_freq_and_mag(&freq_domain)[..]);
    harm_s
}

fn fundamental_freq_and_mag(freq_domain: &Vec<u32>) -> Vec<u32> {
    if let Some(max) = freq_domain.iter().max() {
        vec!((freq_domain.iter().position(|&r| r == *max).unwrap()) as u32, *max)
    } else {
        vec!(0u32, 0)
    }
}

