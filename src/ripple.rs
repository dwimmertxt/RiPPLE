use crate::arg::Commands;
use crate::decode;
use crate::encode;
use crate::export;
use crate::maff::{conv_base_sum, hsl_to_rgb, rgb_to_integer};


pub fn generate(args: Commands) { 
    
    let mut ripple_f: Vec<Vec<u32>> = Vec::new(); // ripple frames; frame = time domain
    
    match args {
        Commands::Ripple {width, height, depth, resolution, save, valve} => {
            for t in 0..resolution {
                ripple_f.push(time_domain(&width, &height, &depth, &t));
            }
            let mut ripple_f_ripl: Vec<u8> = Vec::new();
            if save {
                ripple_f_ripl = encode::ripple(ripple_f, width, height, depth, resolution);
                export::save(&ripple_f_ripl, "ripple");
                if let Ok(data) = decode::ripple() {
                    println!("{:?}", &data[..10]);
                }
            }
            if !valve {
                if ripple_f_ripl.is_empty() {
                    //time_domains_ripl = encode::ripl();
                } else {
                    //export::pipe();
                }
            }
        },
        _ => unreachable!(),
    }
}


pub fn time_domain(width: &u16, height: &u16, depth: &u16, t: &u16) -> Vec<u32> {
    let mut time_domain: Vec<u32> = Vec::new();
    for x in 0..*width {
        let mut amplitudes: Vec<u64> = Vec::new();
        for y in 0..*height {
            for z in 0..*depth {
                // cast values to signed integers to allow subtraction operation 
                amplitudes.push(amplitude(x as i16, y as i16, z as i16, *t as i16));
            }
        }
        let sum: u64 = amplitudes.iter().sum();
        time_domain.push((sum / amplitudes.len() as u64) as u32);
    }
    time_domain
}


pub fn amplitude(x: i16, y: i16, z: i16, t: i16) -> u64 {
    rgb_to_integer(hsl_to_rgb(
        (conv_base_sum(t, 360) * &z - &y * &x).rem_euclid(360) as f64,
        ((conv_base_sum(t, 100) * &z - &y * &x).rem_euclid(100) as f64) * 0.01,
        ((conv_base_sum(t, 100) * &z - &y * &x).rem_euclid(100) as f64) * 0.01,
    )) as u64
}
