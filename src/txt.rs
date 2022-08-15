use std::fs::File;
use std::io::{Write, Error};

use rustfft::{FftPlanner, num_complex::Complex};

use crate::maff::norm_to_audible_freq;


pub fn time_domain(wf_time_domain: &Vec<u32>, norm: &bool) {
    let mut time_domain = Vec::new();
    match *norm {
        true => for (time, amplitude) in wf_time_domain.iter().enumerate() {
            let line = format!("{},{}\n", time, norm_to_audible_freq(*amplitude));
            time_domain.push(line);
        },
        false => for (time, amplitude) in wf_time_domain.iter().enumerate() {
            let line = format!("{},{}\n", time, amplitude);
            time_domain.push(line);
        },
    }

    let time_txt = create_txt_file(time_domain, "time");
    match time_txt {
        Ok(()) => println!("Successfully created time.txt"),
        Err(_) => println!("Error creating time.txt"),
    }
}


pub fn freq_domain(wf_time_domain: &Vec<u32>, norm: &bool) {
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
    };

    let wf_len = wf_freq_domain.len() as usize;
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(wf_len);
    fft.process(&mut wf_freq_domain);
    
    let mut freq_domain = Vec::new();
    for (freq, complex_num) in wf_freq_domain.iter().enumerate() {
        let magnitude = (complex_num.re.powi(2) + complex_num.im.powi(2)).sqrt();
        let line = format!("{},{}\n", freq, magnitude);
        freq_domain.push(line);
    }

    let freq_txt = create_txt_file(freq_domain, "freq");
    match freq_txt {
        Ok(()) => println!("Successfully created freq.txt"),
        Err(_) => println!("Error creating freq.txt"),
    }
}


fn create_txt_file(data: Vec<String>, file_name: &str) -> Result<(), Error> {
    let path = format!("{}.txt", file_name);
    let mut output = File::create(path)?;
    for line in data {
        write!(output, "{}", line)?;
    }
    Ok(())
}