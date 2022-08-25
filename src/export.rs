use std::fs::File;
use std::io::{Write, Error, stdout};

//use crate::arg;
//use std::str;

/*
pub fn prism(args: &arg::Prism, domains_1d: &Vec<Vec<i32>>, domains_2d: &Vec<Vec<Vec<i32>>>) {
    let mut time_jsonl      = Vec::new();
    let mut frequency_jsonl = Vec::new();
    let mut harmonic_jsonl  = Vec::new();

    if args.jsonl {
        for (domain, vec) in domains_1d.iter().enumerate() {
            if !vec.is_empty() {
                match domain {
                    0 => {
                        harmonic_jsonl = jsonl_domain(vec, "harmonic");
                        let _ = save(&args, &harmonic_jsonl, "harmonic");
                    },
                    _ => println!("Here be dragons."),
                }
            }
        }
        for (domain, vec) in domains_2d.iter().enumerate() {
            if !vec.is_empty() {
                match domain {
                    0 => {
                        time_jsonl = jsonl_domains(vec, "time");
                        let _ = save(&args, &time_jsonl, "time");
                    },
                    _ => {
                        frequency_jsonl = jsonl_domains(vec, "frequency");
                        let _ = save(&args, &frequency_jsonl, "frequency");
                    },
                }
            }
        }
    }
    
    if !args.pipe.is_empty() {
        match args.pipe[0].as_str() {
            "t" => {
                if !time_jsonl.is_empty() {
                    let _ = pipe_out(&time_jsonl);
                } else {
                    let _ = pipe_out(& jsonl_domains(&domains_2d[0], "time"));
                }
            },
            "f" => {
                if !frequency_jsonl.is_empty() {
                    let _ = pipe_out(&frequency_jsonl);
                } else {
                    let _ = pipe_out(& jsonl_domains(&domains_2d[1], "frequency"));
                }
            },
            _   => {
                if !harmonic_jsonl.is_empty() {
                    let _ = pipe_out(&harmonic_jsonl);
                } else {
                    let _ = pipe_out(& jsonl_domain(&domains_1d[0], "harmonic"));
                }
            },
        }
    }  
}


fn jsonl_domain(data: &Vec<i32>, domain: &str) -> Vec<String> {
    let domain_len = data.len();
    let mut jsonl = Vec::new();
    let x_field_str;
    let y_field_str;

    match domain {
        "time"      => {
            x_field_str = "Time";
            y_field_str = "Amplitude";
        },
        "frequency" => {
            x_field_str = "Frequency";
            y_field_str = "Magnitude";
        },
        _           => {
            x_field_str = "Time";
            y_field_str = "Frequency";
        },
    }

    for t in 0..domain_len {
        jsonl.push(format!("{{\"{}\": {},", x_field_str, t));
    }

    for (x, y) in data.iter().enumerate() {
        let y_val_str: String = format!("\"{}\": {}}}\n", y_field_str, &y);
        jsonl[x] = format!("{}{}", &jsonl[x], &y_val_str);
    }

    jsonl
}


fn jsonl_domains(data: &Vec<Vec<i32>>, domain: &str) -> Vec<String> {
    let domain_len = data[0].len();
    let mut jsonl = Vec::new();
    let x_field_str;
    let y_field_str;

    
    match domain {
        "time"      => {
            x_field_str = "Time";
            y_field_str = "Amplitude";
        },
        "frequency" => {
            x_field_str = "Frequency";
            y_field_str = "Magnitude";
        },
        _           => {
            x_field_str = "Time";
            y_field_str = "Frequency";
        },
    }

    for t in 0..domain_len {
        jsonl.push(format!("{{\"{}\": {},", x_field_str, t));
    }

    for (t, domain) in data.iter().enumerate() {
        if (t as i32) < (data.len() as i32 - 1) {
            for (x, y) in domain.iter().enumerate() {
                let y_val_str: String = format!("\"R{}_{}\": {},", t, y_field_str, &y);
                jsonl[x] = format!("{}{}", &jsonl[x], &y_val_str);
            }
        } else {
            for (x, y) in domain.iter().enumerate() {
                let y_val_str: String = format!("\"R{}_{}\": {}", t, y_field_str, &y);
                jsonl[x] = format!("{}{}", &jsonl[x], &y_val_str);
            }
        }
        
    }

    for t in 0..domain_len {
        jsonl[t as usize] = format!("{}{}", jsonl[t as usize], "}\n".to_string());
    }
    jsonl.insert(0, format!("{}", "[ ]\n"));
    jsonl
}


fn save(args: &arg::Prism, jsonl: &Vec<String>, domain: &str) -> Result<(), Error> {
    let domain_s;
    match domain {
        "time"      => domain_s = "time_domains",
        "frequency" => domain_s = "frequency_domains",
        _           => domain_s = "harmonic_domain",
    }
    let resolution_s = format!("_resolution={}", args.resolution);
    let width_s = format!("_width={}", args.width);
    let height_s = format!("_height={}", args.height);
    let mut normalise_s = "".to_string();
    if !args.norm.is_empty() {
        normalise_s = format!("_normalise={},{}", args.norm[0], args.norm[1]);
    }
    let args_s = format!("{}{}{}{}{}", domain_s, resolution_s, width_s, height_s, normalise_s);
    let path = format!("{}.jsonl", args_s);
    let mut output = File::create(path)?;
    let mut lines = Vec::new();
    for line in jsonl {
        lines.extend(line.as_bytes());
    }
    output.write_all(&lines[..])?;
    Ok(())
}
*/

pub fn pipe(data: Vec<u8>) -> Result<(), Error> {
    let mut stdout = stdout();
    stdout.write_all(&data[..])?;
    Ok(())
}


pub fn save(data: &Vec<u8>, file_name: &str) -> Result<(), Error> {
    let path = format!("{}.ripl", file_name);
    let mut output = File::create(path)?;
    output.write_all(&data[..])?;
    Ok(())
}