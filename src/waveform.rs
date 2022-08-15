use crate::maff::{conv_base, hsl_to_rgb, rgb_to_integer};


pub fn samples(duration: &u32, sample_rate: &u32) -> Vec<u32> {
    let num_samples = duration * sample_rate;
    let mut samples = Vec::new();
    for t in 0..num_samples {
        let rgb = hsl_to_rgb(
            (conv_base(t, 360) % 360) as f64,
            ((conv_base(t, 100) % 100) as f64) * 0.01,
            ((conv_base(t, 100) % 100) as f64) * 0.01,
        );
        samples.push(rgb_to_integer(rgb[0], rgb[1], rgb[2]));
    }
    samples
}