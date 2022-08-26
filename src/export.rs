use std::fs::File;
use std::io::{Write, Error};


pub fn data_frames(
    data_b: Vec<u8>, file_name: &str, 
    save: bool, shutsave: bool) -> Result<(), Error> {
    
    if save || shutsave {
        let mut output = File::create(file_name)?;
        output.write_all(&data_b[..])?;
    }
    if !shutsave {
        let stdout = std::io::stdout();
        let lock = stdout.lock();
        let mut buf = std::io::BufWriter::new(lock);
        for byte in data_b {
            buf.write(&byte.to_le_bytes())?;
        }
        buf.flush()?;
    }
    Ok(())
}

