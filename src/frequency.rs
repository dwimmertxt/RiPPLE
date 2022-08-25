use rustfft::{FftPlanner, num_complex::Complex};

use crate::encode;
use crate::export;
use crate::import;



pub fn process(save: bool, valve: bool, file: &Vec<String>) { 
    /// process a number of time frames to obtain their respective frequency domains

    let mut time_f: Vec<Vec<u32>> = Vec::new();
    if !file.is_empty() {
        match import::file(file) {
            Ok(data) => time_f = data,
            Err(_) => println!("Error importing file."), 
        }
    } else {
        match import::pipe() {
            Ok(data) => time_f = data,
            Err(_) => println!("Error piping from stdin."), 
        }
    }

    let mut freq_f: Vec<Vec<u32>> = Vec::new(); // frequency frames; frame = frequency domain
    for td in time_f {
        freq_f.push(frequency_domain(&td));
    }

    let freq_f_ripl: Vec<u8> = encode::frequency(freq_f); 
    if save {
        if let Err(_) = export::save(&freq_f_ripl, "freq") {
            println!("Error saving freq_f_ripl to file.");
        }
    }
    if !valve {
        if let Err(_) = export::pipe(freq_f_ripl) {
            println!("Error piping freq_f_ripl to stdout.")
        }
    }
}


pub fn frequency_domain(time_domain: &Vec<u32>) -> Vec<u32> {
    ///

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