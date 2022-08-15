use std::fs::File;
use std::io::{Write, Error};

use rustfft::{FftPlanner, num_complex::Complex};

use crate::maff::norm_to_audible_freq;


pub fn generate_time(waveform_samples: &Vec<u32>) {
    let mut time = 0;
    let mut time_dimension = Vec::new();
    for amplitude in &*waveform_samples {
        let line = format!("{},{}\n", time, amplitude);
        time_dimension.push(line);
        time += 1;
    }
    let time_file = create_txt_file(time_dimension, "time");
    match time_file {
        Ok(()) => println!("Successfully created time.txt"),
        Err(_) => println!("Error creating time.txt"),
    }
}


pub fn generate_freq(waveform_samples: &Vec<u32>) {
    let mut waveform_samples_complex = Vec::new();
    // uncomment analyze uncompressed signal
    //for sample in &*waveform_samples {
    //    waveform_samples_complex.push(Complex{ re: *sample as f32, im: 0.0 });
    //}

    for sample in &*waveform_samples {
        waveform_samples_complex.push(
            Complex{ re: norm_to_audible_freq(*sample) as f32, im: 0.0 });
    }

    let waveform_samples_len = waveform_samples_complex.len() as usize;
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(waveform_samples_len);
    fft.process(&mut waveform_samples_complex);
    
    let mut freq = 0;
    let mut freq_dimension = Vec::new();
    for complex_sample in waveform_samples_complex {
        let magnitude = (complex_sample.re.powi(2) + complex_sample.im.powi(2)).sqrt();
        let line = format!("{},{}\n", freq, magnitude);
        freq_dimension.push(line);
        freq += 1;
    }
    let freq_file = create_txt_file(freq_dimension, "freq");
    match freq_file {
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