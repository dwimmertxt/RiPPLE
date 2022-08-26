use clap::Parser;

use crate::arg::{Args, Commands};
use crate::time;
use crate::frequency;
use crate::harmonic;
use crate::jsonl;


pub fn ripple() {    
    let args = Args::parse();
    match &args.command {
        Commands::Time {width, height, depth, resolution, save, shutsave} => {
            time::generate(*width, *height, *depth, *resolution, *save, *shutsave);
        },
        Commands::Frequency {save, shutsave, file} => {
            frequency::process(*save, *shutsave, file);
        },
        Commands::Harmonic {save, shutsave, file} => {
            harmonic::process(*save, *shutsave, file);
        },
        Commands::Jsonl {save, shutsave, file} => {
            jsonl::process(*save, *shutsave, file);
        },
    }
}