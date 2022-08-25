use clap::Parser;

use crate::arg::{Args, Commands};
use crate::frequency;
use crate::harmonic;
use crate::ripple;


pub fn ripple() {    
    let args = Args::parse();
    match &args.command {
        Commands::Ripple {width, height, depth, resolution, save, valve} => {
            ripple::generate(*width, *height, *depth, *resolution, *save, *valve);
        },
        Commands::Convert {ripl, jsonl, save, valve, time, freq, harm, file} => {

        },
        Commands::Frequency {save, valve, file} => {
            frequency::process(*save, *valve, file);
        },
        Commands::Harmonic {save, valve, file} => {
            harmonic::process(*save, *valve, file);
        },
        _ => unreachable!(),
    }
}