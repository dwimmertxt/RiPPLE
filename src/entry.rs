use clap::Parser;

use crate::arg;
use crate::ripple;


pub fn ripple() {    
    let args = arg::Args::parse();
    match &args.command {
        arg::Commands::Ripple {..} => ripple::generate(args.command),
        _ => println!("no success"),
    }
}