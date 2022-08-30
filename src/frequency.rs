use rustfft::{FftPlanner, num_complex::Complex};


pub fn process(time_frames: &Vec<Vec<u32>>) -> Vec<Vec<u32>> { 
    // process a number of time frames to obtain their respective frequency domains
    let mut freq_frames: Vec<Vec<u32>> = Vec::new();
    for time_domain in time_frames {
        freq_frames.push(frequency_domain(&time_domain));
    }
    freq_frames
}


pub fn frequency_domain(time_domain: &Vec<u32>) -> Vec<u32> {

    let mut fft_pre = Vec::new();
    for amplitude in &*time_domain {
        fft_pre.push(Complex{ re: (*amplitude as f64), im: 0.0});
    }

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(fft_pre.len() as usize);
    fft.process(&mut fft_pre[..]);
    let fft_post = &fft_pre[1..fft_pre.len() / 2];

    let mut freq_domain: Vec<u32> = Vec::new();
    for complex_num in fft_post {
        freq_domain.push((complex_num.re.powi(2) + complex_num.im.powi(2)).sqrt() as u32);
    }
    freq_domain
}