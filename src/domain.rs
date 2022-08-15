use rustfft::{FftPlanner, num_complex::Complex};

use crate::maff::norm_to_audible_freq;

#[derive(Debug)]
pub struct FreqMag {
    pub f:   f32,
    pub m:    f32,
}


pub fn freq_domain(wf_time_domain: &Vec<u32>, norm: &bool) -> Vec<FreqMag> {
    let mut wf_freq_domain = Vec::new();
    match *norm {
        true => for amplitude in &*wf_time_domain {
            wf_freq_domain.push(
                Complex{ re: norm_to_audible_freq(*amplitude) as f32, im: 0.0 });
        },
        false => for amplitude in &*wf_time_domain {
            wf_freq_domain.push(
                Complex{ re: *amplitude as f32, im: 0.0 });
        },
    }

    let wf_len = wf_freq_domain.len() as usize;
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(wf_len);
    fft.process(&mut wf_freq_domain);
    
    let mut freq_domain = Vec::new();
    for (freq, complex_num) in wf_freq_domain.iter().enumerate() {
        let magnitude = (complex_num.re.powi(2) + complex_num.im.powi(2)).sqrt();
        let fm = FreqMag {
            f: freq as f32,
            m: magnitude,
        };
        freq_domain.push(fm);
    }
    freq_domain
}


