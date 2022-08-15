#![feature(exclusive_range_pattern)]

pub mod maff;
pub mod txt;
pub mod wav;
pub mod waveform;

fn main() {
    ripple();
}

//a multi-sensory signal hunting operation
fn ripple() {
    let duration:           u32 = 1;
    let sample_rate:        u32 = 44100;
    
    let waveform_samples = waveform::generate_samples(&duration, &sample_rate);

    wav::generate(&waveform_samples, &sample_rate);
    txt::generate_time(&waveform_samples);
    txt::generate_freq(&waveform_samples);
}
