use crate::read_file;
use image::load_from_memory_with_format;

#[derive(Debug)]
struct internalMeta{
    byte_order: [u8;2],
    tiff_ofs: u32,
    cr2_ver: [u8;2],
    raw_ifd_ofs: u32,
    ifds: Option<[Option<ifd_data>;4]>
}

#[derive(Debug)]
struct ifd_data{
    num_entries: u16,
    ofs: u32,
    entries: Option<Vec<ifd_entry>>,
    next_ifd_ofs: Option<u32>
}

#[derive(Debug)]
struct ifd_entry{
    tag_id: u16,
    pointer: u32
}

fn read_ifd(raw_data: &Vec<u8>, offset:&u32) -> ifd_data{
    let mut data = ifd_data{
        num_entries: 0,
        ofs: *offset,
        entries: Some(vec![]),
        next_ifd_ofs: None
    };
    data.num_entries = u16::from_le_bytes(raw_data[*offset as usize..=(*offset+1) as usize].try_into().unwrap());
    let mut ifd_entries: Vec<ifd_entry> = vec![];
    let last_ofs:usize = (data.ofs + 2+12*data.num_entries as u32) as usize;
    for n in 0..data.num_entries as usize{
        let ifd_ofs:usize;
        if n==0{
            ifd_ofs = data.ofs as usize + 2;
        }
        else {
            ifd_ofs = data.ofs as usize +  2+12*(n);
        }
        let tag_id: u16 = u16::from_le_bytes(raw_data[ifd_ofs..=ifd_ofs+1].try_into().unwrap());
        let tag_pointer: u32 = u32::from_le_bytes(raw_data[ifd_ofs+8..=ifd_ofs+11].try_into().unwrap());
        ifd_entries.push(ifd_entry {tag_id: tag_id, pointer: tag_pointer })
    }
    data.entries.as_mut().unwrap().append(&mut ifd_entries);
    data.next_ifd_ofs = Some(u32::from_le_bytes(raw_data[last_ofs..=last_ofs+3].try_into().unwrap()));
    data
}

fn get_file_header(raw_data: &Vec<u8>)->internalMeta{
    let mut internal_data = internalMeta{
        byte_order: [0,0],
        tiff_ofs: 0,
        cr2_ver: [0,0],
        raw_ifd_ofs: 0,
        ifds: None
    };
    internal_data.byte_order = [raw_data[0], raw_data[1]];
    internal_data.tiff_ofs = u32::from_le_bytes(raw_data[4..=7].try_into().unwrap());
    internal_data.cr2_ver = [raw_data[0xa],raw_data[0xb]];
    internal_data.raw_ifd_ofs = u32::from_le_bytes(raw_data[0xc..=0xf].try_into().unwrap());
    let ifd0 = read_ifd(&raw_data, &internal_data.tiff_ofs);
    let ifd1 = read_ifd(&raw_data, &ifd0.next_ifd_ofs.unwrap());
    let ifd2 = read_ifd(&raw_data, &ifd1.next_ifd_ofs.unwrap());
    let ifd3 = read_ifd(&raw_data, &ifd2.next_ifd_ofs.unwrap());
    internal_data.ifds = Some([Some(ifd0),Some(ifd1),Some(ifd2), Some(ifd3)]);
    internal_data
}

pub fn extract_thumb(file_path: &String, output: &String){
    let raw_data = read_file(file_path);
    let internal_data = get_file_header(&raw_data);
    let mut strip_ofs:u32=0;
    let mut strip_cnt:u32=0;

    let ifd_0_raw = &internal_data.ifds.as_ref().unwrap()[0];
    let ifd_0 = ifd_0_raw.as_ref().unwrap();

    for n in 0..ifd_0.num_entries as usize{
        let entry = &ifd_0.entries.as_ref().unwrap()[n];
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
    let size_factor:f32 = 256.0 / img.width() as f32;
    img = img.thumbnail((img.width() as f32*size_factor)as u32, (img.width() as f32*size_factor)as u32);
    img.save_with_format(output, image::ImageFormat::Png).unwrap();
}