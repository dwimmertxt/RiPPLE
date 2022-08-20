use std::fs::File;
use std::io::{Write, Error, stdout};

use crate::domain;
use crate::export;
use crate::maff;

// (f(t, n) * z - y * x) % n



pub fn additive(resolution: &i32, norm: &Vec<i32>) {
    let x_range: i32 = 32;
    let y_range: i32 = 32;

    let mut fundamental_freqs = Vec::new();
    for x in 0..x_range {
        println!("{:?}", x);
        for y in 0..y_range {
            let time_domain = domain::time(&resolution, &norm, &x, &y);
            let freq_domain = domain::frequency(&time_domain);
            let fundamental_freq = get_fundamental_freq(&freq_domain);
            fundamental_freqs.push(fundamental_freq as i32);
        }
    }
    let fundamental_freqs_csv = export::to_csv(&fundamental_freqs, "frequency");
    let _ = export::save_csv(&fundamental_freqs_csv, "fundamental");
}

fn get_fundamental_freq(freq_domain: &Vec<i32>) -> usize {
    let buffer = &freq_domain[1..];

    for (freq, magnitude) in buffer.iter().enumerate() {

    }
    let max_mag = buffer.iter().max();
    match max_mag {
        Some(max) => {
            let idx = buffer.iter().position(|&r| r == *max).unwrap();
            idx
        },
        None => 0 as usize
    }
}

pub fn additive_tmp(resolution: &i32, norm: &Vec<i32>) {
    let t_range: i32 = 2;
    let x_range: i32 = 4;
    let y_range: i32 = 4;

    let mut time_domains = Vec::new();
    let mut freq_domains = Vec::new();
    let mut fundamental_freqs = Vec::new();
    for t in 0..t_range {
        let mut time_domain = Vec::new();
        for y in 0..y_range {
            for x in 0..x_range {
                let val = time_tmp(&norm, &x, &y, &t);
                println!("{}", val);
                time_domain.push(val);
            }
        }
        println!("{:?}", time_domain);
        time_domains.push(time_domain.clone());
        let freq_domain = domain::frequency(&time_domain);
        freq_domains.push(freq_domain.clone());
        let fundamental_freq = get_fundamental_freq(&freq_domain);
        fundamental_freqs.push(fundamental_freq as i32);
    }
    let time_domains_csv = to_csv(&time_domains, "time");
    let _ = save_csv(&time_domains_csv, "time");
    let fundamental_freqs_csv = export::to_csv(&fundamental_freqs, "frequency");
    let _ = export::save_csv(&fundamental_freqs_csv, "fundamental");
}

pub fn time_tmp(norm: &Vec<i32>, x: &i32, y: &i32, t: &i32) -> i32 {
    let rgb = maff::hsl_to_rgb(
        (maff::conv_base(*t, 360) * 0 - y * x).rem_euclid(360) as f64,
        ((maff::conv_base(*t, 100) * 0 - y * x).rem_euclid(100) as f64) * 0.01,
        ((maff::conv_base(*t, 100) * 0 - y * x).rem_euclid(100) as f64) * 0.01,
    );
    match norm.is_empty() {
        true => {
            maff::rgb_to_integer(&rgb)
        },
        false => {
            maff::normalise(maff::rgb_to_integer(&rgb), &norm)
        },
    }
}


pub fn to_csv(data: &Vec<Vec<i32>>, domain: &str) -> Vec<String> {
    let mut csv = Vec::new();
    for (x, y) in data.iter().enumerate() {
        let mut owned_string: String = format!("{},", x);
        for amplitude in y {
            let borrowed_string = format!("{},", amplitude);
            owned_string.push_str(&borrowed_string);
            
        }
        owned_string.push_str("\n");
        //println!("{},\n", owned_string);
        csv.push(format!("{}", owned_string));
    }
    if domain == "frequency" {
        csv.remove(0);
        csv.insert(0, "frequency,magnitude,\n".to_string())
    } else {
        csv.insert(0, "time, amplitude,\n".to_string())
    }
    csv
}


pub fn save_csv(csv: &Vec<String>, domain: &str) -> Result<(), Error> {
    let path = format!("{}_domain.csv", domain);
    let mut output = File::create(path)?;
    let mut buffer = Vec::new();
    for line in csv {
        buffer.extend(line.as_bytes());
    }
    output.write_all(&buffer[..])?;
    Ok(())
}