

pub fn time(time_f: Vec<Vec<u32>>, 
    width: u32, height: u32, depth: u32, resolution: u32) -> Vec<u8> {

    let mut time_f_b: Vec<u8> = Vec::new();
    time_f_b.extend([82u8, 105, 80, 76]);
    time_f_b.extend([84u8, 73, 77, 69]);
    time_f_b.extend(width.to_le_bytes());
    time_f_b.extend(height.to_le_bytes());
    time_f_b.extend(depth.to_le_bytes());
    time_f_b.extend(resolution.to_le_bytes());
    time_f_b.extend([105u8; 8]);
    time_f_b.extend([68u8, 65, 84, 65]);
    time_f_b.push(3u8);

    for time_d in time_f {
        for amplitude in time_d {
            time_f_b.extend(&amplitude.to_le_bytes()[..3]);
        }
    }
    time_f_b
}


pub fn frequency(freq_f: &Vec<Vec<u32>>, num_f: &usize) -> Vec<u8> {
    let mut freq_f_b: Vec<u8> = Vec::new();
    freq_f_b.extend([82u8, 105, 80, 76]);
    freq_f_b.extend([70u8, 82, 69, 81]);
    freq_f_b.extend((*num_f as u32).to_le_bytes());
    freq_f_b.extend([105u8; 20]);
    freq_f_b.extend([68u8, 65, 84, 65]);
    freq_f_b.push(4u8);

    for freq_d in freq_f {
        for magnitude in freq_d {
            freq_f_b.extend(&magnitude.to_le_bytes());
        }
    }
    freq_f_b
}


pub fn harmonic(_harm_f: &Vec<Vec<u32>>, num_f: &usize) -> Vec<u8> {

    let mut harm_f_b: Vec<u8> = Vec::new();
    harm_f_b.extend([82u8, 105, 80, 76]);
    harm_f_b.extend([72u8, 65, 82, 77]);
    harm_f_b.extend(num_f.to_le_bytes());
    harm_f_b.extend([105u8; 18]);
    harm_f_b.extend([68u8, 65, 84, 65]);
    harm_f_b.push(4u8);
    /*
    for harm_d in harm_f {
        for frequency in harm_d {
            harm_f_b.extend(&frequency.to_le_bytes());
        }
    }
    */
    harm_f_b
}


pub fn jsonl(data: &Vec<Vec<u32>>, id: &str) -> Vec<u8> {
    let data_len = data[0].len();
    let mut jsonl = Vec::new();
    let x_field_str;
    let y_field_str;

    match id {
        "TIME"      => {
            x_field_str = "TIME";
            y_field_str = "AMPL";
        },
        "FREQ" => {
            x_field_str = "FREQ";
            y_field_str = "MAGN";
        },
        _           => {
            x_field_str = "TIME";
            y_field_str = "FREQ";
        },
    }

    for t in 0..data_len {
        jsonl.push(format!("{{\"{}\": {},", x_field_str, t));
    }

    for (t, frame) in data.iter().enumerate() {
        if (t as i32) < (data.len() as i32 - 1) {
            for (x, y) in frame.iter().enumerate() {
                let y_val_str: String = format!("\"f{}_{}\": {},", t, y_field_str, &y);
                jsonl[x] = format!("{}{}", &jsonl[x], &y_val_str);
            }
        } else {
            for (x, y) in frame.iter().enumerate() {
                let y_val_str: String = format!("\"f{}_{}\": {}", t, y_field_str, &y);
                jsonl[x] = format!("{}{}", &jsonl[x], &y_val_str);
            }
        }
        
    }

    for t in 0..data_len {
        jsonl[t as usize] = format!("{}{}", jsonl[t as usize], "}\n".to_string());
    }

    let mut bytes: Vec<u8> = Vec::new();
    for line in jsonl {
        bytes.extend(line.as_bytes());
    }

    bytes
}