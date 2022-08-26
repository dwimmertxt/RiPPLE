use crate::encode;
use crate::export;
use crate::maff::{conv_base_sum, hsl_to_rgb, rgb_to_integer};


pub fn generate(
    width: u32, height: u32, depth: u32, resolution: u32, 
    save: bool, shutsave: bool) { 
    // generate a number of time frames
    
    let mut time_f: Vec<Vec<u32>> = Vec::new(); // ripple frames; frame = time domain
    for t in 0..resolution {
        time_f.push(time_domain(&width, &height, &depth, &t));
    }
    let time_f_ripl: Vec<u8> = encode::time(time_f, width, height, depth, resolution);  
    if let Err(export_err) = export::data_frames(
        time_f_ripl, "TIME.ripl", save, shutsave) {

        eprintln!("{:?}", export_err);
    }
}


pub fn time_domain(width: &u32, height: &u32, depth: &u32, t: &u32) -> Vec<u32> {
    let mut time_domain: Vec<u32> = Vec::new();
    for x in 0..*width {
        let mut amplitudes: Vec<u64> = Vec::new();
        for y in 0..*height {
            for z in 0..*depth {
                // cast values to signed integers to allow subtraction operation 
                amplitudes.push(amplitude(x as i32, y as i32, z as i32, *t as i32));
            }
        }
        let sum: u64 = amplitudes.iter().sum();
        time_domain.push((sum / amplitudes.len() as u64) as u32);
    }
    time_domain
}


pub fn amplitude(x: i32, y: i32, z: i32, t: i32) -> u64 {
    rgb_to_integer(hsl_to_rgb(
        (conv_base_sum(t, 360) * &z - &y * &x).rem_euclid(360) as f64,
        ((conv_base_sum(t, 100) * &z - &y * &x).rem_euclid(100) as f64) * 0.01,
        ((conv_base_sum(t, 100) * &z - &y * &x).rem_euclid(100) as f64) * 0.01,
    )) as u64
}
