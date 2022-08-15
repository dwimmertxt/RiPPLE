#![feature(exclusive_range_pattern)]

use clap::Parser;

pub mod maff;
pub mod txt;
pub mod wav;
pub mod waveform;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// a multi-sensory signal hunting operation
struct Args {
    #[clap(short, long, value_parser, default_value_t = 1, help = "Duration in seconds")]
    duration:       u32,
    #[clap(short, long, value_parser, default_value_t = 44100, help = "Sample rate in Hz")]
    sample_rate:    u32,
    #[clap(short, long, takes_value = false, help = "Generate freq.txt")]
    freq:           bool,
    #[clap(short, long, takes_value = false, help = "Generate time.txt")]
    time:           bool,
    #[clap(short, long, takes_value = false, help = "Generate ripple.wav")]
    wav:            bool,
    #[clap(short, long, takes_value = false, help = "Normalise to 20-20kHz")]
    norm:           bool,
}


fn main() {
    ripple(Args::parse());
}


fn ripple(args: Args) {
    let wf_time_domain = waveform::samples(
        &args.duration, &args.sample_rate);

    if args.wav {wav::generate(&wf_time_domain, &args.sample_rate)};
    if args.time {txt::time_domain(&wf_time_domain, &args.norm)};
    if args.freq {txt::freq_domain(&wf_time_domain, &args.norm)};
}
