use crate::encode;
use crate::export;
use crate::import;



pub fn process(save: bool, valve: bool, file: &Vec<String>) { 
    /// process a number of frequency frames to obtain their respective harmonic series'

    let mut freq_f: Vec<Vec<u32>> = Vec::new();
    if !file.is_empty() {
        match import::file(file) {
            Ok(data) => freq_f = data,
            Err(_) => println!("Error importing file."), 
        }
    } else {
        match import::pipe() {
            Ok(data) => freq_f = data,
            Err(_) => println!("Error piping from stdin."), 
        }
    }

    let mut harm_f: Vec<Vec<u32>> = Vec::new(); // harmonic frames; frame = harmonic series
    for fd in freq_f {
        harm_f.push(harmonic_series(&fd));
    }

    let harm_f_ripl: Vec<u8> = encode::harmonic(harm_f);
    if save {
        if let Err(_) = export::save(&harm_f_ripl, "harm") {
            println!("Error saving harm_f_ripl to file.");
        }
    }
    if !valve {
        if let Err(_) = export::pipe(harm_f_ripl) {
            println!("Error piping harm_f_ripl to stdout.")
        }
    }
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

