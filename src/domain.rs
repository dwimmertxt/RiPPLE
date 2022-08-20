use rustfft::{FftPlanner, num_complex::Complex};
use crate::maff::{conv_base, hsl_to_rgb, normalise, rgb_to_integer};

pub fn time(resolution: &i32, norm: &Vec<i32>, x: &i32, y: &i32) -> Vec<i32> {
    let mut time_domain = Vec::new();
    for t in 0..*resolution {
        let rgb = hsl_to_rgb(
            ((conv_base(t + x, 360) * conv_base(t + y, 360)) % 360) as f64,
            (((conv_base(t + x, 100) * conv_base(t + y, 100)) % 100) as f64) * 0.01,
            (((conv_base(t + x, 100) * conv_base(t + y, 100)) % 100) as f64) * 0.01,
        );
        match norm.is_empty() {
            true => {
                time_domain.push(rgb_to_integer(&rgb));
            },
            false => {
                time_domain.push(normalise(rgb_to_integer(&rgb), &norm));
            },
        }
    }
    time_domain
}


pub fn frequency(time_domain: &Vec<i32>) -> Vec<i32> {
    let mut fft_buffer = Vec::new();

    for amplitude in &*time_domain {
        fft_buffer.push(Complex{ re: (*amplitude as f64), im: 0.0});
    }

    let fft_buffer_len = fft_buffer.len() as usize;
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(fft_buffer_len);
    fft.process(&mut fft_buffer[..]);
    
    let fft_result = &fft_buffer[0..fft_buffer.len() / 2];

    let mut freq_domain = Vec::new();
    for complex_num in fft_result {
        let magnitude = (complex_num.re.powi(2) + complex_num.im.powi(2)).sqrt();
        freq_domain.push(magnitude as i32);
    }
    freq_domain
}


