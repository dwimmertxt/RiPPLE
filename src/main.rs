#![feature(exclusive_range_pattern)]
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Vec<u8> {
    // h [0, 360.0], s [0, 1.0], l [0, 1.0]
    let c = (1.0 - f64::abs((2.0 * l) - 1.0)) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - f64::abs((h_prime % 2.0) - 1.0));
    let m = l - (c / 2.0);
    let rgb = match h_prime as i32 {
        0..1 => vec![c + m, x + m, m],
        1..2 => vec![x + m, c + m, m],
        2..3 => vec![m, c + m, x + m],
        3..4 => vec![m, x + m, c + m],
        4..5 => vec![x + m, m, c + m],
        _    => vec![c + m, m, x + m],
    };
    vec![
        (rgb[0] * 255.0) as u8,
        (rgb[1] * 255.0) as u8,
        (rgb[2] * 255.0) as u8,
    ]
}

fn rgb_to_decimal(r: u8, g: u8, b: u8) -> i32 {
    (65536 * r as i32) + (256 * g as i32) + b as i32
}

fn main() {
    let rgb = hsl_to_rgb(126.0, 1.0, 0.75);
    println!("{:?}", rgb);
    let rgb_as_decimal = rgb_to_decimal(rgb[0], rgb[1], rgb[2]);
    println!("{}", rgb_as_decimal);
}
