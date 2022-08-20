use clap::Parser;

use crate::additive;
use crate::arg::RippleArgs;
use crate::domain;
use crate::export;
use crate::wav;


pub fn ripple() {    
    let args = RippleArgs::parse();

    let time_domain = domain::time(&args.resolution, &args.norm, &0, &0);

    additive::additive_tmp(&args.resolution, &args.norm);

    let mut freq_domain = Vec::new();
    if args.fft {freq_domain = domain::frequency(&time_domain)};
    // analyse fft
    export::entry(&time_domain, &freq_domain, &args);

}