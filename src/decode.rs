use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn ripple() -> std::io::Result<Vec<u8>> {
    let mut f = File::open("ripple.ripl")?;
    let mut reader = BufReader::new(f);

    let mut bytes: Vec<u8> = Vec::new(); 
    for byte in reader.bytes() {
        bytes.push(byte.unwrap());
    }
    //println!("{:?}", &bytes[..10]);
    Ok(bytes)
}