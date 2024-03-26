use crate::read_file;



pub fn extract_thumb(file_path: &String, output: &String){
    let raw_file = read_file(&file_path);
    let byte_order: [u8;2] = [raw_file[0], raw_file[1]];
    let header_length: u32 = u32::from_le_bytes(raw_file[2..=5].try_into().unwrap());
    let dir_start_0 = u32::from_le_bytes(raw_file[raw_file.len()-4..=raw_file.len()-1].try_into().unwrap());

    println!("byte_order: {:?}\nheader_length: {}\ndir_start_0: {}, calc_offset: {}",byte_order, header_length, dir_start_0, (header_length + dir_start_0))
}