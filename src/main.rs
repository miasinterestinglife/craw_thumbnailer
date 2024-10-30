use clap::Parser;
use std::fs;
mod cr2;
mod cr3;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short,long)]
    file: String,

    #[arg(short,long)]
    output: String,

    #[arg(short,long,default_value_t=0)]
    size:u16
}

#[derive(Debug)]
struct InternalMeta{
    byte_order: [u8;2],
    tiff_ofs: u32,
    cr2_ver: [u8;2],
    raw_ifd_ofs: u32,
    ifds: Option<[Option<IFDData>;4]>
}

#[derive(Debug)]
struct IFDData{
    num_entries: u16,
    ofs: u32,
    entries: Option<Vec<IFDEntry>>,
    next_ifd_ofs: Option<u32>
}

#[derive(Debug)]
struct IFDEntry{
    tag_id: u16,
    pointer: u32
}

fn read_ifd(raw_data: &Vec<u8>, offset:&u32, internal_data: &InternalMeta) -> IFDData{
    let mut data = IFDData{
        num_entries: 0,
        ofs: *offset,
        entries: Some(vec![]),
        next_ifd_ofs: None
    };
    data.num_entries = bytes_to_u16(&raw_data[*offset as usize..=(*offset+1) as usize], &internal_data.byte_order);
    let mut ifd_entries: Vec<IFDEntry> = vec![];
    let last_ofs:usize = (data.ofs + 2+12*data.num_entries as u32) as usize;
    for n in 0..data.num_entries as usize{
        let ifd_ofs:usize;
        if n==0{
            ifd_ofs = data.ofs as usize + 2;
        }
        else {
            ifd_ofs = data.ofs as usize +  2+12*(n);
        }
        let tag_id = bytes_to_u16(&raw_data[ifd_ofs..=ifd_ofs+1], &internal_data.byte_order);
        let tag_pointer = bytes_to_u32(&raw_data[ifd_ofs+8..=ifd_ofs+11], &internal_data.byte_order);
        ifd_entries.push(IFDEntry {tag_id: tag_id, pointer: tag_pointer })
    }
    data.entries.as_mut().unwrap().append(&mut ifd_entries);
    data.next_ifd_ofs = Some(bytes_to_u32(&raw_data[last_ofs..=last_ofs+3], &internal_data.byte_order));
    data
}

fn get_file_header(raw_data: &Vec<u8>)->InternalMeta{
    let mut internal_data = InternalMeta{
        byte_order: [0,0],
        tiff_ofs: 0,
        cr2_ver: [0,0],
        raw_ifd_ofs: 0,
        ifds: None
    };
    internal_data.byte_order = [raw_data[0], raw_data[1]];
    internal_data.tiff_ofs = bytes_to_u32(&raw_data[4..=7], &internal_data.byte_order);
    internal_data.cr2_ver = [raw_data[0xa],raw_data[0xb]];
    internal_data.raw_ifd_ofs = bytes_to_u32(&raw_data[0xc..=0xf], &internal_data.byte_order);
    let ifd0 = read_ifd(&raw_data, &internal_data.tiff_ofs, &internal_data);
    internal_data.ifds = Some([Some(ifd0),None,None,None]);
    internal_data
}

fn read_file(file_path: &String) -> Vec<u8>{
    let data = fs::read(file_path).unwrap();
    data
}

fn bytes_to_u32(bytes: &[u8], endianness: &[u8;2])->u32{
    if endianness == &[0x49, 0x49]{
        (bytes[0] as u32)
    | ((bytes[1] as u32)<<8)
    | ((bytes[2] as u32)<<16)
    | ((bytes[3] as u32)<<24)
    }
    else if endianness == &[0x4d,0x4d]{
        ((bytes[0] as u32) << 24)
    | ((bytes[1] as u32) << 16)
    | ((bytes[2] as u32) << 8)
    | (bytes[3] as u32)
    }
    else{
        return 0 as u32
    }
}

fn bytes_to_u16(bytes: &[u8], endianness: &[u8;2])->u16{
    if endianness == &[0x49,0x49]{
        (bytes[0] as u16)
    | ((bytes[1] as u16)<<8)
    }
    else if endianness == &[0x4d,0x4d]{
        ((bytes[0] as u16) << 8)
    | (bytes[1] as u16)
    }
    else{
        return 0 as u16
    }
}


fn main() {
    let args = Args::parse();
    let input = args.file;
    let output = args.output;
    let size = args.size;
    if input.ends_with("CR2")|| input.ends_with("cr2"){
        cr2::extract_thumb(&input, &output, size);
    }
    if input.ends_with("CR3")|| input.ends_with("cr3"){
        cr3::extract_thumb(&input, &output, size)
    }
}
