pub fn conv_base_sum(mut n: i16, b: i16) -> i16 {
    // converts number n into base b
    // and returns sum of its digits
    let mut digits = vec![0];
    while n > 0 {
        digits.push(n.rem_euclid(b));
        n = n / b;
    }
    digits.iter().sum()
}


pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
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


pub fn normalise(val: i32, norm: &Vec<i32>) -> i32 {
    // (x - r_min) / (r_max - r_min) * (t_max - t_min) + t_min
    // r [0, 16777216]
    // t [20, 20000]
    let x     = val     as f64;
    let t_min = norm[0] as f64;
    let t_max = norm[1] as f64;
    (x  / 16777216.0 * (t_max - t_min) + t_min) as i32
}


pub fn rgb_to_integer(rgb: [u8; 3]) -> u64 {
    (65536 * (rgb[0] as u64)) + (256 * (rgb[1] as u64)) + (rgb[2] as u64)
}