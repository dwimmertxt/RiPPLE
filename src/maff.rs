pub fn conv_base(mut n: u32, b: u32) -> u32 {
    // converts number n into base b
    // then returns converted num modulo b
    let mut digits = vec![0];
    while n > 0 {
        digits.push(n % b);
        n = n / b;
    }
    digits.iter().sum()
}


pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [u32; 3] {
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
        (rgb[0] * 255.0) as u32,
        (rgb[1] * 255.0) as u32,
        (rgb[2] * 255.0) as u32,
    ]
}


pub fn norm_to_audible_freq(x: u32) -> i16 {
    // (x - r_min) / (r_max - r_min) * (t_max - t_min) + t_min
    // r [0, 16777216]
    // t [20, 20000]
    ((x as f64) / 16777216.0 * 22050.0 + 0.0).round() as i16
}


pub fn rgb_to_integer(r: u32, g: u32, b: u32) -> u32 {
    (65536 * r) + (256 * g) + b
}