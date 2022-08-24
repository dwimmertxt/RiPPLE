#![feature(exclusive_range_pattern)]

pub mod arg;
pub mod decode;
pub mod encode;
pub mod entry;
pub mod export;
pub mod frequency;
pub mod harmonic;
pub mod maff;
pub mod ripple;
pub mod wav;


fn main() {
    entry::ripple()
}




