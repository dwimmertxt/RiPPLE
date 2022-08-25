pub fn ripple(frames: Vec<Vec<u32>>, 
    width: u16, height: u16, depth: u16, resolution: u16) -> Vec<u8> {
    ///

    let mut frames_b: Vec<u8> = Vec::new();
    frames_b.extend("RiPL".as_bytes());
    frames_b.extend("TIME".as_bytes());
    frames_b.extend(width.to_le_bytes());
    frames_b.extend(height.to_le_bytes());
    frames_b.extend(depth.to_le_bytes());
    frames_b.extend(resolution.to_le_bytes());
    frames_b.extend("DATA".as_bytes());
    frames_b.push(3u8);
    for time_domain in frames {
        for amplitude in time_domain {
            frames_b.extend(&amplitude.to_le_bytes()[..3]);
        }
    }
    frames_b
}


pub fn frequency(freq_f: Vec<Vec<u32>>) -> Vec<u8> {
    /// 

    let mut frames_b: Vec<u8> = Vec::new();
    frames_b
}


pub fn harmonic(freq_f: Vec<Vec<u32>>) -> Vec<u8> {
    /// 

    let mut frames_b: Vec<u8> = Vec::new();
    frames_b
}