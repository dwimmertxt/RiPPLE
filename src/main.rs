#![feature(exclusive_range_pattern)]

pub mod arg;
pub mod entry;
pub mod import;
pub mod export;
pub mod encode;
pub mod decode;
pub mod jsonl;
pub mod maff;
pub mod time;
pub mod frequency;
pub mod harmonic;
pub mod wav;


fn main() {
    entry::ripple()
}




