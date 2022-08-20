use std::fs::File;
use std::io::{Write, Error, stdout};

use crate::arg::RippleArgs;
use crate::maff::normalise;
use crate::wav;

pub fn entry(time_domain: &Vec<i32>, freq_domain: &Vec<i32>, args: &RippleArgs) {
    if args.wav {wav::generate(&time_domain)};

    let mut time_domain_csv = Vec::new();
    let mut freq_domain_csv = Vec::new();
    if !args.csv.is_empty() {
        for domain in &args.csv {
            match domain.as_str() {
                "t" => {
                    time_domain_csv = to_csv(&time_domain, &"time");
                    let _ = save_csv(&time_domain_csv, &"time")
                    .expect("Could not create .csv");
                },
                "f" => {
                    if args.fft {
                        freq_domain_csv = to_csv(&freq_domain, &"frequency");
                        let _ = save_csv(&freq_domain_csv, &"frequency")
                        .expect("Could not create.csv");
                    } else {
                        println!("Set --freq flag to generate frequency domain for export.");
                    }
                },
                &_  => {},
            }
        }  
    }
    if !args.pipe.is_empty() {
        match args.bytes {
            true => {
                match args.pipe[0].as_str() {
                    "t" => {
                        //let _ = pipe_bytes(&time_domain)
                        //.expect("Could not pipe time domain bytes.");
                    },
                    "f" => {
                        if args.fft {
                            //let _ = pipe_bytes(&freq_domain)
                            //.expect("Could not pipe frequency domain bytes.");
                        } else {
                            //println!("Set --freq flag to generate frequency domain for export.");
                        }
                    },
                    _ => {},
                }
            },
            false => {
                match args.pipe[0].as_str() {
                    "t" => {
                        let _ = if time_domain_csv.is_empty() {
                            pipe_csv(&to_csv(&time_domain, &"time"))
                        } else {
                            pipe_csv(&time_domain_csv)
                        };
                    },
                    "f" => {
                        if args.fft {
                            let _ = if freq_domain_csv.is_empty() {
                                let tmp = to_csv(&freq_domain, &"frequency");
                                pipe_csv(&tmp)
                            } else {
                                pipe_csv(&freq_domain_csv)
                            };
                        }
                    },
                    _ => {},
                }
            },
        }
    }
}


pub fn to_csv(data: &Vec<i32>, domain: &str) -> Vec<String> {
    let mut csv = Vec::new();
    for (x, y) in data.iter().enumerate() {
        csv.push(format!("{},{},\n", x, y));
    }
    if domain == "frequency" {
        csv.remove(0);
        csv.insert(0, "frequency,magnitude,\n".to_string())
    } else {
        csv.insert(0, "time, amplitude,\n".to_string())
    }
    csv
}


pub fn save_csv(csv: &Vec<String>, domain: &str) -> Result<(), Error> {
    let path = format!("{}_domain.csv", domain);
    let mut output = File::create(path)?;
    let mut buffer = Vec::new();
    for line in csv {
        buffer.extend(line.as_bytes());
    }
    output.write_all(&buffer[..])?;
    Ok(())
}


fn pipe_csv(csv: &Vec<String>) -> Result<(), Error> {
    let mut stdout = stdout();
    let mut buffer = Vec::new();
    for line in csv {
        buffer.extend(line.as_bytes());
    }
    stdout.write_all(&buffer[..])?;
    Ok(())
}

/*
fn pipe_bytes(data: &Vec<i32>) -> Result<(), Error> {
    let mut bytes = Vec::new();
    for value in data {
        bytes.extend(value.to_le_bytes());
    }
    stdout().write_all(&bytes[..]);
    Ok(())
}*/
