use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::borrow::Cow;

pub fn ripl() -> std::io::Result<Vec<Vec<u32>>> {
    let f = File::open("ripple.ripl")?;
    
    // read first eight bytes
    let mut header = [0; 8];
    f.take(8).read(&mut header)?;
    let f_type = String::from_utf8_lossy(&header[..4]);
    let f_meta = String::from_utf8_lossy(&header[4..]);
    
    if f_type == "RiPL" {
        match f_meta {
            Cow::Borrowed("TIME") => {},//time_func,
            Cow::Borrowed("FREQ") => println!("FREQ"),
            Cow::Borrowed("HARM") => println!("HARM"),
            _      => println!("Error: corrupt file."),
        }
    } else {
        println!("Error: provided file isn't ripl format.")
    }

    /*
    let mut reader = BufReader::new(f);

    let mut buf: Vec<u8> = Vec::new();
    reader.read_until(4u8, &mut buf);
    println!("result: {:?}", buf);
    /*
    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    */
    
    */
    let ripple_f: Vec<Vec<u32>> = Vec::new();
    
    Ok(ripple_f)
}
/*
fn time() -> Vec<Vec<u32>> {
    let mut reader = BufReader::new(f);

}*/