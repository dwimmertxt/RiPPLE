use clap::Parser;

use crate::arg;
use crate::prism;


pub fn ripple() {    
    let prism_args = arg::Prism::parse();

    prism::generate(&prism_args);
}