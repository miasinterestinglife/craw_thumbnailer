use crate::read_file;
use crate::bytes_to_u32;
use image::load_from_memory_with_format;


pub fn extract_thumb(file_path: &String, output: &String){
    let raw_file = read_file(&file_path);
    let index = raw_file.windows(4).position(|w| w==[0x50,0x52,0x56,0x57]).unwrap()-4;
    let jpeg_size = bytes_to_u32(&raw_file[index+20..=index+23], &[0x4d,0x4d]);
    let raw_img = &raw_file[index+24..=index+24+jpeg_size as usize];
    let mut img = load_from_memory_with_format(raw_img, image::ImageFormat::Jpeg).unwrap();
    let size_factor:f32 = 256.0 / img.width() as f32;
    img = img.thumbnail((img.width() as f32*size_factor)as u32, (img.width() as f32*size_factor)as u32);
    img.save_with_format(output, image::ImageFormat::Png).unwrap();
}