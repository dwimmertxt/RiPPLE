pub fn ripple_1d(ux: &u32, ut: &u32) -> u32 {
    let (x, t) = (*ux as i64, *ut as i64); 
    rgb_to_integer(hsl_to_rgb(
        (conv_base_sum(t, 360) * x).rem_euclid(360) as f64,
        ((conv_base_sum(t, 100) * x).rem_euclid(100) as f64) * 0.01,
        ((conv_base_sum(t, 100) * x).rem_euclid(100) as f64) * 0.01,
    ))
}


pub fn ripple_2d(ux: &u32, uy: &u32, ut: &u32) -> u32 {
    let (x, y, t) = (*ux as i64, *uy as i64, *ut as i64); 
    rgb_to_integer(hsl_to_rgb(
        (conv_base_sum(t, 360) * y * x).rem_euclid(360) as f64,
        ((conv_base_sum(t, 100) * y * x).rem_euclid(100) as f64) * 0.01,
        ((conv_base_sum(t, 100) * y * x).rem_euclid(100) as f64) * 0.01,
    )) 
}


pub fn ripple_3d(ux: &u32, uy: &u32, uz: &u32, ut: &u32) -> u32 {
    let (x, y, z, t) = (*ux as i64, *uy as i64, *uz as i64, *ut as i64); 
    rgb_to_integer(hsl_to_rgb(
        (conv_base_sum(t, 360) * z - y * x).rem_euclid(360) as f64,
        ((conv_base_sum(t, 100) * z - y * x).rem_euclid(100) as f64) * 0.01,
        ((conv_base_sum(t, 100) * z - y * x).rem_euclid(100) as f64) * 0.01,
    )) 
}


fn rgb_to_integer(rgb: [u8; 3]) -> u32 {
    (65536 * (rgb[0] as u32)) + (256 * (rgb[1] as u32)) + (rgb[2] as u32)
}


fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
    // h [0.0, 360.0], s [0.0, 1.0], l [0.0, 1.0]
    let c = (1.0 - f64::abs((2.0 * l) - 1.0)) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - f64::abs((h_prime % 2.0) - 1.0));
    let m = l - (c / 2.0);
    let rgb = match h_prime as i32 {
        0..1 => [c + m, x + m, m],
        1..2 => [x + m, c + m, m],
        2..3 => [m, c + m, x + m],
        3..4 => [m, x + m, c + m],
        4..5 => [x + m, m, c + m],
        _ => [c + m, m, x + m],
    };
    [
        (rgb[0] * 255.0) as u8,
        (rgb[1] * 255.0) as u8,
        (rgb[2] * 255.0) as u8,
    ]
}


fn conv_base_sum(mut n: i64, b: i64) -> i64 {
    // converts number n into base b
    // and returns sum of its digits
    let mut digits = vec![0];
    while n > 0 {
        digits.push(n.rem_euclid(b));
        n = n / b;
    }
    digits.iter().sum()
}