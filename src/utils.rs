use std::fs;

pub fn get_index() -> u32 {
    let entries = fs::read_dir("wasm").unwrap();
    let mut highest_num = u32::MIN;

    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.path().file_name() {
                if let Some(filename_str) = filename.to_str() {
                    let filename_vec: Vec<&str> = filename_str.split(".").collect();
                    let filename_num = filename_vec.first().unwrap().parse::<u32>().unwrap();
                    if filename_num > highest_num {
                        highest_num = filename_num;
                    }
                }
            }
        }
    }

    highest_num + 1
}
