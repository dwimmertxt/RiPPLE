use std::fs::File;
use std::io::{BufWriter, Error, Write, stdout};

use atty::{is, Stream};


pub fn data_frames(file_name: &str, data_frames: &Vec<u8>, save: &bool) -> Result<(), Error> {  
    if *save {
        let mut output = File::create(file_name)?;
        output.write_all(&data_frames[..])?;
    }
    if !is(Stream::Stdout) {
        let stdout = stdout();
        let lock = stdout.lock();
        let mut buf = BufWriter::new(lock);
        for byte in data_frames {
            buf.write(&byte.to_le_bytes())?;
        }
        buf.flush()?;
    }
    Ok(())
}

