use std::fs::File;
use std::io::{Write, Error};

use crate::domain::FreqMag;
use crate::maff::norm_to_audible_freq;

pub fn freq(freq_domain: &Vec<FreqMag>) {
    let mut lines = Vec::new();
    for fm in freq_domain {
        lines.push(format!("{},{:?}\n", fm.f, fm.m));
    }
    
    let freq_txt = save(lines, "freq");
    match freq_txt {
        Ok(()) => println!("Successfully created freq.txt"),
        Err(_) => println!("Error creating freq.txt"),
    }
}


pub fn time(wf_time_domain: &Vec<u32>, norm: &bool) {
    let mut lines = Vec::new();
    match *norm {
        true => for (time, amplitude) in wf_time_domain.iter().enumerate() {
            lines.push(format!("{},{}\n", time, norm_to_audible_freq(*amplitude)));
        },
        false => for (time, amplitude) in wf_time_domain.iter().enumerate() {
            lines.push(format!("{},{}\n", time, amplitude));
        },
    }
    
    let time_txt = save(lines, "time");
    match time_txt {
        Ok(()) => println!("Successfully created time.txt"),
        Err(_) => println!("Error creating time.txt"),
    }
}


fn save(data: Vec<String>, file_name: &str) -> Result<(), Error> {
    let path = format!("{}.txt", file_name);
    let mut output = File::create(path)?;
    for line in data {
        write!(output, "{}", line)?;
    }
    Ok(())
}