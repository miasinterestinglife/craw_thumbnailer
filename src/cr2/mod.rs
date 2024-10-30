use crate::read_file;
use crate::get_file_header;
use crate::{IFDData, IFDEntry};
use image::load_from_memory_with_format;

pub fn extract_thumb(file_path: &String, output: &String, size: u16){
    let raw_data = read_file(file_path);
    let internal_data = get_file_header(&raw_data);
    let mut strip_ofs:u32=0;
    let mut strip_cnt:u32=0;

    let ifd_0_raw: &Option<IFDData> = &internal_data.ifds.as_ref().unwrap()[0];
    let ifd_0: &IFDData = ifd_0_raw.as_ref().unwrap();

    for n in 0..ifd_0.num_entries as usize{
        let entry: &IFDEntry = &ifd_0.entries.as_ref().unwrap()[n];
        match entry.tag_id{
            273 => {
                strip_ofs = entry.pointer;
            }
            279 => {
                strip_cnt = entry.pointer
            }
            _ => {}
        }
    }
    let raw_img = &raw_data[strip_ofs as usize..=strip_ofs as usize+strip_cnt as usize];
    let mut img = load_from_memory_with_format(raw_img, image::ImageFormat::Jpeg).unwrap();
    let size_factor:f32;
    if size != 0{
        size_factor = size as f32 / img.width() as f32;
    }
    else{
        size_factor = 1.0;
    }
    img = img.thumbnail((img.width() as f32*size_factor)as u32, (img.width() as f32*size_factor)as u32);
    img.save_with_format(output, image::ImageFormat::Png).unwrap();
}