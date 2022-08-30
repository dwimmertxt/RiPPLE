use std::io::{BufReader, Read, Error, ErrorKind, stdin};

use atty::{is, Stream};

use crate::decode;


pub fn data_frames() -> Result<(&'static str, Vec<Vec<u32>>), Error> {
    match is(Stream::Stdin) {
        true => Err(Error::new(ErrorKind::Other, "no data provided")),
        false => {
            let stdin = stdin();
            let lock = stdin.lock();
            let mut buf = BufReader::new(lock);
            let mut bytes = Vec::new();
            buf.read_to_end(&mut bytes)?;
            Ok(decode::ripl(bytes))
            }, 
    }
}

