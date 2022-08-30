use crate::encode;
use crate::export;
use crate::maff::{ripple_1d, ripple_2d, ripple_3d};


pub fn generate(width: u32, square: bool, cube: bool, frames: u32, save: bool) { 
    // generate a number of time frames
    let (time_frames, dimensionality) = match (square, cube) {
        (false, false) => (time_domain(&width, &frames), 0u8),
        (true, false)  => (square_time_domain(&width, &frames), 1u8),
        (false, true)  => (cube_time_domain(&width, &frames), 2u8),
        _              => unreachable!(),
    };

    let time_ripl: Vec<u8> = encode::time(&time_frames, &width, &dimensionality, &frames);  
    if let Err(export_err) = export::data_frames("TIME.ripl", &time_ripl, &save) {
        eprintln!("{:?}", export_err);
    }
}


fn time_domain(width: &u32, frames: &u32) -> Vec<Vec<u32>> {
    let mut time_frames: Vec<Vec<u32>> = Vec::new();
    for t in 0..*frames {
        let mut amplitudes: Vec<u32> = Vec::new();
        for x in 0..*width { 
            // cast values to signed integers to allow subtraction operation
            // suspicion: final value halved due to casting to i32. perhaps i64 instead? 
            amplitudes.push(ripple_1d(&x, &t));
        }
        time_frames.push(amplitudes.clone());
    }
    //println!("{:?}", time_frames);
    time_frames 
}


fn square_time_domain(width: &u32, frames: &u32) -> Vec<Vec<u32>> {
    let mut time_frames: Vec<Vec<u32>> = Vec::new();
    for t in 0..*frames {
        let mut time_domain: Vec<u32> = Vec::new();
        for x in 0..*width {
            let mut amplitudes: Vec<u64> = Vec::new();
            for y in 0..*width {
                // cast values to signed integers to allow subtraction operation 
                amplitudes.push(ripple_2d(&x, &y, &t) as u64);
            }
            let sum: u64 = amplitudes.iter().sum();
            time_domain.push((sum / amplitudes.len() as u64) as u32);
        }
        time_frames.push(time_domain.clone());
    }
    time_frames
}


fn cube_time_domain(width: &u32, frames: &u32) -> Vec<Vec<u32>> {
    let mut time_frames: Vec<Vec<u32>> = Vec::new();
    for t in 0..*frames {
        let mut time_domain: Vec<u32> = Vec::new();
        for x in 0..*width {
            let mut amplitudes: Vec<u64> = Vec::new();
            for y in 0..*width {
                for z in 0..*width {
                    // cast values to signed integers to allow subtraction operation 
                    amplitudes.push(ripple_3d(&x, &y, &z, &t) as u64);
                }
            }
            let sum: u64 = amplitudes.iter().sum();
            time_domain.push((sum / amplitudes.len() as u64) as u32);
        }
        time_frames.push(time_domain.clone());
    }
    time_frames
}