use crate::maff::{conv_base, hsl_to_rgb, normalise, rgb_to_integer};


pub fn domain(norm: &Vec<i32>, width: &i32, height: &i32, t: i32) -> Vec<i32> {
    let mut time_domain = Vec::new();
    for y in 0..*height {
        for x in 0..*width {
            time_domain.push(amplitude(norm, x, y, None, &t));
        }
    }
    time_domain 
}


pub fn amplitude(norm: &Vec<i32>, x: i32, y: i32, z: Option<i32>, t: &i32) -> i32 {
    let rgb = hsl_to_rgb(
        (conv_base(*t, 360) * z.unwrap_or(1) - &y * &x).rem_euclid(360) as f64,
        ((conv_base(*t, 100) * z.unwrap_or(1) - &y * &x).rem_euclid(100) as f64) * 0.01,
        ((conv_base(*t, 100) * z.unwrap_or(1) - &y * &x).rem_euclid(100) as f64) * 0.01,
    );
    match norm.is_empty() {
        true => rgb_to_integer(rgb),
        false => normalise(rgb_to_integer(rgb), &norm),
    }
}