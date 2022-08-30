pub fn time(
    time_frames: &Vec<Vec<u32>>, width: &u32, dimensionality: &u8, frames: &u32) -> Vec<u8> {
    let mut ripl_bytes: Vec<u8> = Vec::new();
    ripl_bytes.extend([82u8, 105, 80, 76]);     // 04 bytes; ASCII: "RiPL"
    ripl_bytes.extend([84u8, 73, 77, 69]);      // 04 bytes; ASCII: "TIME"
    ripl_bytes.extend(width.to_le_bytes());     // 04 bytes;   u32: width
    ripl_bytes.push(*dimensionality);           // 01 bytes;    u8: time domain dimensionality
    ripl_bytes.extend(frames.to_le_bytes());    // 04 bytes;   u32: frames
    ripl_bytes.extend([105u8; 15]);             // 15 bytes; ASCII: "i", reserved for expansion
    ripl_bytes.extend([68u8, 65, 84, 65]);      // 04 bytes; ASCII: "DATA"
    ripl_bytes.push(3u8);                       // 01 bytes;    u8: bytes per datum

    for frame in time_frames {
        for amplitude in frame {
            ripl_bytes.extend(&amplitude.to_le_bytes()[..3]);
        }
    }
    ripl_bytes
}


pub fn frequency(freq_frames: &Vec<Vec<u32>>) -> Vec<u8> {
    let frame_len = freq_frames[0].len() as u32;
    let mut ripl_bytes: Vec<u8> = Vec::new();
    ripl_bytes.extend([82u8, 105, 80, 76]);     // 04 bytes; ASCII: "RiPL"
    ripl_bytes.extend([70u8, 82, 69, 81]);      // 04 bytes; ASCII: "FREQ"
    ripl_bytes.extend(frame_len.to_le_bytes()); // 04 bytes;   u32: data per frame
    ripl_bytes.extend([105u8; 20]);             // 20 bytes; ASCII: "i", reserved for expansion
    ripl_bytes.extend([68u8, 65, 84, 65]);      // 04 bytes; ASCII: "DATA"
    ripl_bytes.push(4u8);                       // 01 bytes;    u8; bytes per datum

    for frame in freq_frames {
        for magnitude in frame {
            ripl_bytes.extend(&magnitude.to_le_bytes());
        }
    }
    ripl_bytes
}


pub fn harmonic(harm_frames: &Vec<Vec<u32>>) -> Vec<u8> {
    let frame_len = harm_frames[0].len() as u32;
    let mut ripl_bytes: Vec<u8> = Vec::new();
    ripl_bytes.extend([82u8, 105, 80, 76]);     // 04 bytes; ASCII: "RiPL"
    ripl_bytes.extend([72u8, 65, 82, 77]);      // 04 bytes; ASCII: "HARM"
    ripl_bytes.extend(frame_len.to_le_bytes()); // 04 bytes;   u32: data per frame
    ripl_bytes.extend([105u8; 20]);             // 20 bytes; ASCII: "i", reserved for expansion
    ripl_bytes.extend([68u8, 65, 84, 65]);      // 04 bytes; ASCII: "DATA"
    ripl_bytes.push(4u8);                       // 01 bytes;    u8: bytes per datum
    
    for frame in harm_frames {
        for frequency in frame {
            ripl_bytes.extend(&frequency.to_le_bytes());
        }
    }
    ripl_bytes
}


pub fn jsonl(id: &str, data: &Vec<Vec<u32>>) -> Vec<u8> {
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