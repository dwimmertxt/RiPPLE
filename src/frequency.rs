use rustfft::{FftPlanner, num_complex::Complex};

use crate::encode;
use crate::export;
use crate::import;



pub fn process(save: bool, shutsave: bool, file: &Vec<String>) { 
    // process a number of time frames to obtain their respective frequency domains
    match import::data_frames(&file) {
        Ok(time_d) => {
            let (_id, time_f) = time_d;
            let mut freq_f: Vec<Vec<u32>> = Vec::new();
            for td in time_f {
                freq_f.push(frequency_domain(&td));
            }
            let freq_f_ripl: Vec<u8> = encode::frequency(&freq_f, &freq_f[0].len());
            if let Err(export_err) = export::data_frames(
                freq_f_ripl, "FREQ.ripl", save, shutsave) {
                
                eprintln!("ERR: failed to export data frames in Frequency.\n{:?}", export_err);   
            }
        },
        Err(import_err) => eprintln!("ERR: failed to import data frames in Frequency.\n{:?}", import_err),
    }
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