use crate::import;
use crate::export;
use crate::encode;


pub fn process(save: bool, shutsave: bool, file: &Vec<String>) { 
    // process a number of time frames to obtain their respective frequency domains

    match import::data_frames(&file) {
        Ok(data_d) => {
            let (id, data_f) = data_d;
            let data_f_jsonl: Vec<u8> = encode::jsonl(&data_f, id);
            if let Err(export_err) = export::data_frames(
                data_f_jsonl, &format!("{}.jsonl", id), save, shutsave) {
                
                eprintln!("{:?}", export_err);
            }
        },
        Err(import_err) => eprintln!("{:?}", import_err), 
    }
}