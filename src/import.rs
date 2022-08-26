use std::io::{BufReader, Read, Error, ErrorKind, stdin};

use crate::decode;


pub fn data_frames(file: &Vec<String>) -> Result<(&str, Vec<Vec<u32>>), Error> {

    match file.is_empty() {
        true => {
            let mut stdin = BufReader::new(stdin());
            let mut bytes = Vec::new();
            stdin.read_to_end(&mut bytes)?;
            Ok(decode::ripl(bytes)) 
        },
        false => {
            match std::fs::read(&file[0]) {
                Ok(bytes) => {
                    match std::str::from_utf8(&bytes[..4]).unwrap() {
                        "RiPL" => Ok(decode::ripl(bytes)),
                        _      => Err(Error::new(ErrorKind::Other, 
                            "file is not .ripl format")),
                    }
                },
                Err(error) => Err(error),
            } 
        },
    }
}

