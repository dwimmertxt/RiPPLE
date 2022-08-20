use rustfft::{FftPlanner, num_complex::Complex};



pub fn domain(time_domain: &Vec<i32>) -> Vec<i32> {
    let mut fft_pre = Vec::new();
    for amplitude in &*time_domain {
        fft_pre.push(Complex{ re: (*amplitude as f64), im: 0.0});
    }
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(fft_pre.len() as usize);
    fft.process(&mut fft_pre[..]);
    
    let fft_post = &fft_pre[1..fft_pre.len() / 2];

    let mut frequency_domain = Vec::new();
    for complex_num in fft_post {
        frequency_domain.push((complex_num.re.powi(2) + complex_num.im.powi(2)).sqrt() as i32);
    }
    frequency_domain
}