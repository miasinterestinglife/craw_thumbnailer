use crate::read_file;
use crate::get_file_header;
use crate::save_image;
use crate::{IFDData, IFDEntry};
use std::io::{Error, ErrorKind};

pub fn extract_thumb(file_path: &String, output: &String, size: u16)-> Result<(), Error>{
    let raw_data = read_file(file_path)?;
    let internal_data = get_file_header(&raw_data);
    let mut strip_ofs:u32=0;
    let mut strip_cnt:u32=0;

    let ifd_0: &IFDData;
    match &internal_data.ifds{
        Some(ifds) => match &ifds[0]{
            Some(ifd) => ifd_0 = &ifd,
            _ => {return Err(Error::new(ErrorKind::NotFound, "Unable to find IFD0, file may be corrupted"))}
        }
        _ => {return Err(Error::new(ErrorKind::NotFound, "Unable to find the list of IFDs, file may be corrupted"));}
    }

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
    save_image(raw_img, output, size, 1)?;
    Ok(())
}