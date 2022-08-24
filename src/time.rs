use crate::maff::{conv_base, hsl_to_rgb, rgb_to_integer};


pub fn domain(width: &u16, height: &u16, depth: &u16, t: &u16) -> Vec<u32> {
    let mut time_domain: Vec<u32> = Vec::new();
    for x in 0..*width {
        let mut amplitudes: Vec<u64> = Vec::new();
        for y in 0..*height {
            for z in 0..*depth {
                amplitudes.push(amplitude(x, y, z, *t));
            }
        }
        let sum: u64 = amplitudes.iter().sum();
        time_domain.push((sum / amplitudes.len() as u64) as u32);
    }
    time_domain
}


pub fn amplitude(x: u16, y: u16, z: u16, t: u16) -> u64 {
    rgb_to_integer(hsl_to_rgb(
        (conv_base_sum(t, 360) * &z - &y * &x).rem_euclid(360) as f64,
        ((conv_base_sum(t, 100) * &z - &y * &x).rem_euclid(100) as f64) * 0.01,
        ((conv_base_sum(t, 100) * &z - &y * &x).rem_euclid(100) as f64) * 0.01,
    )) as i64
}