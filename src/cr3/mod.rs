use crate::{read_file, get_file_header};
use crate::bytes_to_u32;
use image::load_from_memory_with_format;

pub fn extract_thumb(file_path: &String, output: &String, size: u16){
    let raw_file = read_file(&file_path);
    let index = raw_file.windows(4).position(|w| w==[0x50,0x52,0x56,0x57]).unwrap()-4;
    let jpeg_size: u32 = bytes_to_u32(&raw_file[index+20..=index+23], &[0x4d,0x4d]);
    let raw_img: &[u8] = &raw_file[index+24..=index+24+jpeg_size as usize];
    let mut img = load_from_memory_with_format(raw_img, image::ImageFormat::Jpeg).unwrap();

    //get Metadata to show correct Orientation
    let exif_index = raw_file.windows(4).position(|w| w==[0x43, 0x4D, 0x54, 0x31]).unwrap()+4;
    let exif_end_idx = raw_file.windows(4).position(|w| w==[0x43, 0x4D, 0x54, 0x32]).unwrap()-1;
    let raw_exif: &[u8] = &raw_file[exif_index..=exif_end_idx];
    let exif_internal_data = get_file_header(&raw_exif.to_vec());

    for n in 0..exif_internal_data.ifds.as_ref().unwrap()[0].as_ref().unwrap().num_entries as usize{
        let entry = &exif_internal_data.ifds.as_ref().unwrap()[0].as_ref().unwrap().entries.as_ref().unwrap()[n];
        if entry.tag_id == 274{
            match entry.pointer{
                /*
                1: rotate 0 degrees
                6: rotate 90 degrees
                3: rotate 180 degrees
                8: rotate 270 degrees 
                */
                6 => img = img.rotate90(),
                3 => img = img.rotate180(),
                8 => img = img.rotate270(),
                _ => {}
            }
        }
    }

    let size_factor:f32;
    if size != 0{
        size_factor = size as f32 / img.width() as f32;
    }
    else{
        size_factor=1.0
    }
    img = img.thumbnail((img.width() as f32*size_factor)as u32, (img.width() as f32*size_factor)as u32);
    img.save_with_format(output, image::ImageFormat::Png).unwrap();
}