use crate::encode;
use crate::export;
use crate::import;



pub fn process(save: bool, shutsave: bool, file: &Vec<String>) { 
    // process a number of frequency frames to obtain their respective harmonic series'

    match import::data_frames(&file) {
        Ok(freq_d) => {
            let (_id, freq_f) = freq_d;
            let mut harm_f: Vec<Vec<u32>> = Vec::new(); // harmonic frames; frame = harmonic series
            for fd in freq_f {
                harm_f.push(harmonic_series(&fd));
            }
            let harm_f_ripl: Vec<u8> = encode::harmonic(&harm_f, &harm_f.len());
            if let Err(export_err) = export::data_frames(
                harm_f_ripl, "HARM.ripl", save, shutsave) {

                eprintln!("{:?}", export_err);
            }
        },
        Err(import_err) => eprintln!("{:?}", import_err),
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

