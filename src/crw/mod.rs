use std::io::{Error, ErrorKind};

use crate::{bytes_to_u16, bytes_to_u32, read_file, save_image};

#[derive(Debug)]
struct DirEntry{
    tag_id: u16,
    size: u32,
    offset: u32
}
impl DirEntry{
    pub fn new()->DirEntry{
        return DirEntry { tag_id: 0, size: 0, offset: 0 }
    }
}

#[derive(Debug)]
struct DBS{
    dir_count: u16,
    dir_entries: Vec<DirEntry>
}
impl DBS{
    pub fn new()->DBS{
        return DBS { dir_count: 0, dir_entries: vec![] }
    }
}

fn parse_root_dir(raw_data: &Vec<u8>)->Result<DBS, Error>{
    let mut dbs = DBS::new();
    let endianness = &[raw_data[0], raw_data[1]];
    let header_length = bytes_to_u32(&raw_data[2..=5], endianness);
    let root_dir_location = bytes_to_u32(&raw_data[raw_data.len()-4..=raw_data.len()-1], endianness) + header_length;

    dbs.dir_count = bytes_to_u16(&raw_data[root_dir_location as usize..=root_dir_location as usize+1], endianness);
    if root_dir_location+dbs.dir_count as u32*10>raw_data.len() as u32{
        return Err(Error::new(ErrorKind::Other, "Root Directory is out of bounds."))
    }
    let dir_data_start = root_dir_location+2;
    for i in 0..dbs.dir_count{
        let mut dir_entry = DirEntry::new();

        let entry_offset = dir_data_start as usize + (i as usize * 10);
        dir_entry.tag_id = bytes_to_u16(&raw_data[entry_offset..=entry_offset+1], endianness);
        dir_entry.size = bytes_to_u32(&raw_data[entry_offset+2..=entry_offset+5], endianness);
        dir_entry.offset = bytes_to_u32(&raw_data[entry_offset+6..=entry_offset+9], endianness);
        dbs.dir_entries.push(dir_entry);
    }

    Ok(dbs)
}

fn get_thumbnail_data(raw_data: &Vec<u8>)->Result<(u32,u32), Error>{
    let endianness = &[raw_data[0], raw_data[1]];
    let header_length = bytes_to_u32(&raw_data[2..=5], endianness);
    let mut thumb_location: u32 = u32::MAX;
    let mut thumb_size: u32 = u32::MAX;

    let root_dir = parse_root_dir(&raw_data)?;
    for entry in root_dir.dir_entries{
        if entry.tag_id == 0x2007{
            thumb_location = entry.offset;
            thumb_size = entry.size;
        }
    }
    if thumb_location==u32::MAX && thumb_size == u32::MAX{
        return Err(Error::new(ErrorKind::NotFound, "Could not find the Directory Entry for the Thumbnail File."))
    }
    Ok((thumb_location + header_length, thumb_size))
}

pub fn extract_thumb(file_path: &String, output: &String, size: u16) -> Result<(),Error>{
    let raw_data = read_file(&file_path)?;
    let (thumb_loc,thumb_size) = get_thumbnail_data(&raw_data)?;

    save_image(&raw_data[thumb_loc as usize..(thumb_loc +thumb_size) as usize], output, size, 1)?;
    Ok(())
}