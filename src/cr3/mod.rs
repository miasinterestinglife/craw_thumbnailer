
use crate::{get_file_header, read_file, save_image, bytes_to_u32, CRAWError};

const PRVW_HEADER: [u8; 4] = [0x50, 0x52, 0x56, 0x57]; // PRVW header
const CMT1_HEADER: [u8; 4] = [0x43, 0x4D, 0x54, 0x31]; // CMT1 header
const CMT2_HEADER: [u8; 4] = [0x43, 0x4D, 0x54, 0x32]; // CMT2 header

pub fn extract_thumb(file_path: &String, output: &String, size: u16) -> Result<(),CRAWError>{
    let raw_file = read_file(&file_path)?;
    //find the location of the PRVW (0x50 52 56 57) header, defining the position of the full-size JPEG
    let index = match raw_file.windows(4).position(|w| w==PRVW_HEADER){
        Some(idx) => {
            if idx >= 4{
                idx-4
            }
            else{
                return Err(CRAWError::CR3PRVWHeaderIndexError(idx));
            }
        },
        _ => return Err(CRAWError::CR3PRVWHeaderNotFound)
    };

    //get length of JPEG Data and extract it
    let jpeg_size: u32 = bytes_to_u32(&raw_file[index+20..index+24], &[0x4d,0x4d]);
    let jpeg_start = index+24;
    let jpeg_end = jpeg_start+jpeg_size as usize;
    let raw_img: &[u8];
    if jpeg_end<=raw_file.len(){
        raw_img = &raw_file[jpeg_start..jpeg_end];
    }
    else{
        return Err(CRAWError::JPEGOutOfBoundsError { detected_size: jpeg_size, start: jpeg_start, file_length: raw_file.len() })
    }

    //get Metadata to show correct Orientation
    let mut exif_index = 0;
    let mut exif_end_idx = 0;
    //find CMT1 (0x43 4D 54 31) header, beginning of Exif IFD 0
    match raw_file.windows(4).position(|w| w==CMT1_HEADER){
        Some(index) => {
            exif_index = index+4;
            //CMT2 comes right after CMT1, so it is used to find the end of the Exif IFD0
            match raw_file.windows(4).position(|w| w==CMT2_HEADER){
                Some(index) => {
                    exif_end_idx = index-1;
                }
                _ => {}
            }
        },
        _ => {}
    }
    //set a default orientation in case there is no usable exif data
    let mut orientation=1;
    if exif_index != 0 && exif_end_idx != 0 && exif_index<exif_end_idx{
        //extract the raw exif IFD
        let raw_exif: &[u8] = &raw_file[exif_index..=exif_end_idx];
        let exif_internal_data = get_file_header(&raw_exif.to_vec())?;
    
        //search for the Exif Entry with the ID 274, containing the Orientation
        let ifd0 = match &exif_internal_data.ifds{
            Some(ifd_list) => match &ifd_list[0]{
                Some(ifd) => ifd,
                None => return Err(CRAWError::IFDDataEntriesNotDefined)
            },
            None => return Err(CRAWError::IFDDataEntriesNotDefined)
        };
        for n in 0..ifd0.num_entries as usize{
            let entry = match &ifd0.entries{
                Some(entries) => &entries[n],
                None => return Err(CRAWError::IFDDataEntriesNotDefined)
            };
            if entry.tag_id == 274{
                orientation = entry.pointer;
            }
        }
    }
    

    save_image(raw_img, output, size, orientation)?;
    Ok(())
}