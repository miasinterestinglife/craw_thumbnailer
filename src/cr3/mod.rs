use crate::{get_file_header, read_file, save_image};
use crate::bytes_to_u32;
use std::io::{Error, ErrorKind};

pub fn extract_thumb(file_path: &String, output: &String, size: u16) -> Result<(),Error>{
    let raw_file = read_file(&file_path)?;

    //find the location of the PICT header, defining the position of the full-size JPEG
    let index = match raw_file.windows(4).position(|w| w==[0x50,0x52,0x56,0x57]){
        Some(idx) => idx-4,
        _ => return Err(Error::new(ErrorKind::NotFound, "PICT Header not found, file may be corrupted."))
    };
    let jpeg_size: u32 = bytes_to_u32(&raw_file[index+20..=index+23], &[0x4d,0x4d]);
    let raw_img: &[u8] = &raw_file[index+24..=index+24+jpeg_size as usize];

    //get Metadata to show correct Orientation
    let mut exif_index = 0;
    let mut exif_end_idx = 0;
    match raw_file.windows(4).position(|w| w==[0x43, 0x4D, 0x54, 0x31]){
        Some(index) => {
            exif_index = index+4;
            match raw_file.windows(4).position(|w| w==[0x43, 0x4D, 0x54, 0x32]){
                Some(index) => {
                    exif_end_idx = index-1;
                }
                _ => {}
            }
        },
        _ => {}
    }
    let mut orientation=1;
    //if the exif is not found, it should still be possible to output the thumbnail,
    //even if it might be the wrong orientation
    if exif_index != 0 && exif_end_idx != 0{
        let raw_exif: &[u8] = &raw_file[exif_index..=exif_end_idx];
        let exif_internal_data = get_file_header(&raw_exif.to_vec());
    
        for n in 0..exif_internal_data.ifds.as_ref().unwrap()[0].as_ref().unwrap().num_entries as usize{
            let entry = &exif_internal_data.ifds.as_ref().unwrap()[0].as_ref().unwrap().entries.as_ref().unwrap()[n];
            if entry.tag_id == 274{
                orientation = entry.pointer;
            }
        }
    }
    

    save_image(raw_img, output, size, orientation)?;
    Ok(())
}