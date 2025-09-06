use std::io::{Error, ErrorKind};

use crate::{read_file, save_image};

const JPEG_HEADER: [u8;4] = [0xFF, 0xD8, 0xFF, 0xDB];
const JPEG_FOOTER: [u8;2] = [0xFF, 0xD9];

pub fn extract_thumb(file_path: &String, output: &String, size: u16) -> Result<(),Error>{
    let raw_data = read_file(&file_path)?;
    let mut jpeg_header_indices: Vec<usize> = vec![];
    let mut jpeg_footer_idx: usize = usize::MAX;

    //since i was unable to reliably read the file, we're taking the dumb approach (works, just not as efficient)
    for (i,window) in raw_data.windows(JPEG_HEADER.len()).enumerate(){
        if window==JPEG_HEADER{
            jpeg_header_indices.push(i);
        }
    }
    //we're looking for the second JPEG as the order is as follows: 
    //  first image is the raw image, 
    //  second image is the embedded jpeg, 
    //  third image is the thumbnail (very low-res)
    if jpeg_header_indices.len()<2{
        return Err(Error::new(ErrorKind::NotFound, "Did not find the embedded Preview JPEG, file may be corrupted."))
    }
    for(i,window) in raw_data.windows(JPEG_FOOTER.len()).enumerate().skip(jpeg_header_indices[1]){
        if window==JPEG_FOOTER{
            jpeg_footer_idx=i+1;
            break;
        }
    }

    if jpeg_footer_idx == usize::MAX{
        return Err(Error::new(ErrorKind::NotFound, "The End of the embedded JPEG could not be found, this suggests you might have a corrupted or unsupported File."))
    }
    save_image(&raw_data[jpeg_header_indices[1]..=jpeg_footer_idx], output, size, 1)?;
    Ok(())
}