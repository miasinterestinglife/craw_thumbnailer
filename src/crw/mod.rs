use crate::{bytes_to_u16, bytes_to_u32, read_file, save_image, CRAWError};

const CRW_MAGIC_NUM: [u8;9] = [0x48, 0x45, 0x41, 0x50, 0x43, 0x43, 0x44, 0x52, 0x02];

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

fn parse_root_dir(raw_data: &Vec<u8>)->Result<DBS, CRAWError>{
    let mut dbs = DBS::new();
    let endianness = &[raw_data[0], raw_data[1]];
    let header_length = bytes_to_u32(&raw_data[2..=5], endianness);
    let root_dir_location = bytes_to_u32(&raw_data[raw_data.len()-4..=raw_data.len()-1], endianness) + header_length;

    if &raw_data[0x06..=0x0E] != CRW_MAGIC_NUM{
        return Err(CRAWError::FileTypeError)
    }

    dbs.dir_count = bytes_to_u16(&raw_data[root_dir_location as usize..=root_dir_location as usize+1], endianness);
    if root_dir_location+dbs.dir_count as u32*10>raw_data.len() as u32{
        return Err(CRAWError::RootDirOutOfBoundsError)
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

fn get_rotation(raw_data: &Vec<u8>) -> Result<u32, CRAWError>{
    //this is higly suboptimal and a bad practice, but I will need to refactor this code anyway
    let mut root_dir = DBS::new();
    let endianness = &[raw_data[0], raw_data[1]];
    let header_length = bytes_to_u32(&raw_data[2..=5], endianness);
    let root_dir_location = bytes_to_u32(&raw_data[raw_data.len()-4..=raw_data.len()-1], endianness) + header_length;

    if &raw_data[0x06..=0x0E] != CRW_MAGIC_NUM{
        return Err(CRAWError::FileTypeError)
    }

    root_dir.dir_count = bytes_to_u16(&raw_data[root_dir_location as usize..=root_dir_location as usize+1], endianness);
    if root_dir_location+root_dir.dir_count as u32*10>raw_data.len() as u32{
        return Err(CRAWError::RootDirOutOfBoundsError)
    }
    let dir_data_start = root_dir_location+2;
    for i in 0..root_dir.dir_count{
        let mut dir_entry = DirEntry::new();

        let entry_offset = dir_data_start as usize + (i as usize * 10);
        dir_entry.tag_id = bytes_to_u16(&raw_data[entry_offset..=entry_offset+1], endianness);
        dir_entry.size = bytes_to_u32(&raw_data[entry_offset+2..=entry_offset+5], endianness);
        dir_entry.offset = bytes_to_u32(&raw_data[entry_offset+6..=entry_offset+9], endianness);
        root_dir.dir_entries.push(dir_entry);
    }

    let mut dir300a_ofs = 0_usize;
    let mut dir300a_size = 0_usize;
    for entry in root_dir.dir_entries{
        if entry.tag_id == 0x300a{
            dir300a_ofs = (entry.offset + header_length)as usize;
            dir300a_size = entry.size as usize;
        }
    }
    let mut dir300a = DBS::new();
    let dir300a_dirstart  = bytes_to_u32(&raw_data[(dir300a_ofs+dir300a_size-4)..=(dir300a_ofs+dir300a_size)], endianness) as usize;
    dir300a.dir_count = bytes_to_u16(&raw_data[(dir300a_ofs+dir300a_dirstart) as usize..=(dir300a_ofs+dir300a_dirstart+1) as usize], endianness);
    let dir300a_datastart = dir300a_ofs+dir300a_dirstart+2;
    for i in 0..dir300a.dir_count{
        let mut dir_entry = DirEntry::new();

        let entry_offset = dir300a_datastart + (i as usize*10);
        dir_entry.tag_id = bytes_to_u16(&raw_data[entry_offset..=entry_offset+1], endianness);
        dir_entry.size = bytes_to_u32(&raw_data[entry_offset+2..=entry_offset+5], endianness);
        dir_entry.offset = bytes_to_u32(&raw_data[entry_offset+6..=entry_offset+9], endianness);
        dir300a.dir_entries.push(dir_entry);
    }

    let mut image_info_ofs = 0_usize;
    for entry in dir300a.dir_entries{
        if entry.tag_id == 0x1810{
            image_info_ofs = entry.offset as usize + dir300a_ofs;
        }
    }
    return match raw_data.get(image_info_ofs+16){
        Some(orientation) => Ok(*orientation as u32),
        None => Err(CRAWError::OrientationNotFound)
    }
}

fn get_thumbnail_data(raw_data: &Vec<u8>)->Result<(u32,u32, u32), CRAWError>{
    let endianness = &[raw_data[0], raw_data[1]];
    let header_length = bytes_to_u32(&raw_data[2..=5], endianness);
    let mut thumb_location: u32 = u32::MAX;
    let mut thumb_size: u32 = u32::MAX;
    let orientation = get_rotation(raw_data)?;

    let root_dir = parse_root_dir(&raw_data)?;
    for entry in root_dir.dir_entries{
        if entry.tag_id == 0x2007{
            thumb_location = entry.offset;
            thumb_size = entry.size;
        }
        if entry.tag_id == 0x300a{
            
        }
    }
    if thumb_location==u32::MAX && thumb_size == u32::MAX{
        return Err(CRAWError::ThumbnailIfdEntryNotFound)
    }
    Ok((thumb_location + header_length, thumb_size, orientation))
}

pub fn extract_thumb(file_path: &String, output: &String, size: u16) -> Result<(),CRAWError>{
    let raw_data = read_file(&file_path)?;
    let (thumb_loc,thumb_size, orientation) = get_thumbnail_data(&raw_data)?;

    save_image(&raw_data[thumb_loc as usize..(thumb_loc +thumb_size) as usize], output, size, orientation)?;
    Ok(())
}