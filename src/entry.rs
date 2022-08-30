use clap::Parser;

use crate::arg::{Args, Commands};
use crate::time;
use crate::frequency;
use crate::harmonic;
use crate::import;
use crate::export;
use crate::encode;
use crate::wav;


pub fn ripple() {
    //println!("{:?}", is(Stream::Stdin));

    let args = Args::parse();
    match &args.command {
        Commands::Time {width, square, cube, frames, save} => {
            time::generate(*width, *square, *cube, *frames, *save);
        },
        Commands::Frequency {save} => transform(*save),
        Commands::Harmonic {save} => transform(*save),
        Commands::Jsonl {save} => convert(*save, "jsonl"),
        Commands::Wav {save} => convert(*save, "wav"),
    }
}


fn transform(save: bool) { 
    match import::data_frames() {
        Ok(data_frames) => {
            let (in_id, in_frames) = data_frames;

            let (out_id, out_frames) = match in_id {
                "TIME" => ("FREQ", encode::frequency(&frequency::process(&in_frames))),
                "FREQ" => ("HARM", encode::harmonic(&harmonic::process(&in_frames))),
                _      => ("NULL", Vec::new()),
            };
            let file_name = format!("{}.{}", out_id, "ripl");
            if let Err(exp_err) = export::data_frames(&file_name, &out_frames, &save) {
                eprintln!("ERR: failed to export data frames.\n{:?}", exp_err);   
            }
        },
        Err(imp_err) => eprintln!("ERR: failed to import data frames.\n{:?}", imp_err),
    }
}


fn convert(save: bool, fmt: &str) { 
    match import::data_frames() {
        Ok(data_frames) => {
            let (id, frames) = data_frames;

            let encoded = match fmt {
                "jsonl" => encode::jsonl(&id, &frames),
                "wav"   => wav::encode(&id, &frames),
                _       => unreachable!(), 
            };
            let file_name = format!("{}.{}", id, fmt);
            if let Err(exp_err) = export::data_frames(&file_name, &encoded, &save) {
                eprintln!("ERR: failed to export data frames.\n{:?}", exp_err);   
            }
        },
        Err(imp_err) => eprintln!("ERR: failed to import data frames.\n{:?}", imp_err),
    }
}