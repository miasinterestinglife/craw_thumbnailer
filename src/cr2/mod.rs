use crate::read_file;
use crate::get_file_header;
use crate::save_image;
use crate::{IFDData, CRAWError};

pub fn extract_thumb(file_path: &String, output: &String, size: u16)-> Result<(), CRAWError>{
    let raw_data = read_file(file_path)?;
    let internal_data = get_file_header(&raw_data)?;
    let mut img_start:u32=0;
    let mut strip_cnt:u32=0;

    let ifd_0: &IFDData;
    match &internal_data.ifds{
        Some(ifds) => match &ifds[0]{
            Some(ifd) => ifd_0 = &ifd,
            _ => {return Err(CRAWError::IFD0NotFound)}
        }
        _ => {return Err(CRAWError::IFDListNotFound);}
    }

    for n in 0..ifd_0.num_entries as usize{
        let entry = match &ifd_0.entries{
            Some(entries) => &entries[n],
            None => return Err(CRAWError::IFDDataEntriesNotDefined)
        };
        println!("{}",entry.tag_id);
        match entry.tag_id{
            273 => {
                //offset of the embedded jpeg
                img_start = entry.pointer;
            }
            279 => {
                //number of pixels of embedded jpeg
                strip_cnt = entry.pointer
            }
            _ => {}
        }
    }

    if img_start==0||strip_cnt==0{
        return Err(CRAWError::CR2ImageDataNotFound { strip_ofs: img_start, strip_count: strip_cnt });
    }
    let img_end = img_start + strip_cnt;
    //extract data from beginning of offset to the end (strip_ofs+strip_cnt)
    let raw_img = &raw_data[img_start as usize..img_end as usize];
    save_image(raw_img, output, size, 1)?;
    Ok(())
}