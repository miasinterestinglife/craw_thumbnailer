use clap::Parser;
use std::fs;
use std::io::{Error, ErrorKind};
use image::load_from_memory_with_format;
mod crw;
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
impl InternalMeta{
    pub fn new() -> InternalMeta{
        return InternalMeta { byte_order: [0,0], tiff_ofs: 0, cr2_ver: [0,0], raw_ifd_ofs: 0, ifds: None }
    }
}

#[derive(Debug)]
struct IFDData{
    num_entries: u16,
    ofs: u32,
    entries: Option<Vec<IFDEntry>>,
    next_ifd_ofs: Option<u32>
}
impl IFDData{
    pub fn new()->IFDData{
        return IFDData{num_entries: 0, ofs: 0, entries: Some(vec![]), next_ifd_ofs: None};
    }
}

//the second field is called pointer because it usually points to data but can also contain data itself, depending on the Tag
#[derive(Debug)]
struct IFDEntry{
    tag_id: u16,
    pointer: u32
}

fn read_ifd(raw_data: &Vec<u8>, offset:&u32, byte_order: &[u8;2]) -> IFDData{
    //!Reads the IFD (Image File Directory) in the TIFF-like CR2 file (also works for normal TIFF files, but usage may vary)
    let mut data = IFDData::new();
    data.ofs = *offset;
    data.num_entries = bytes_to_u16(&raw_data[*offset as usize..=(*offset+1) as usize], byte_order);
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
        let tag_id = bytes_to_u16(&raw_data[ifd_ofs..=ifd_ofs+1], byte_order);
        let tag_pointer = bytes_to_u32(&raw_data[ifd_ofs+8..=ifd_ofs+11], byte_order);
        ifd_entries.push(IFDEntry {tag_id: tag_id, pointer: tag_pointer })
    }
    data.entries.as_mut().unwrap().append(&mut ifd_entries);
    data.next_ifd_ofs = Some(bytes_to_u32(&raw_data[last_ofs..=last_ofs+3], byte_order));
    data
}

fn get_file_header(raw_data: &Vec<u8>)->InternalMeta{
    //!Get TIFF file header, made for use with CR2 files, partially works with regular TIFF
    let mut internal_data = InternalMeta::new();
    internal_data.byte_order = [raw_data[0], raw_data[1]];
    internal_data.tiff_ofs = bytes_to_u32(&raw_data[4..=7], &internal_data.byte_order);
    internal_data.cr2_ver = [raw_data[0xa],raw_data[0xb]];
    internal_data.raw_ifd_ofs = bytes_to_u32(&raw_data[0xc..=0xf], &internal_data.byte_order);
    let ifd0: IFDData = read_ifd(&raw_data, &internal_data.tiff_ofs, &internal_data.byte_order);
    internal_data.ifds = Some([Some(ifd0),None,None,None]);
    internal_data
}

fn read_file(file_path: &String) -> Result<Vec<u8>, Error>{
    //!Read the file from specified path and return a Result<Vec<u8>>
    let data = fs::read(file_path);
    match data{
        Ok(bytes) => return Ok(bytes),
        Err(error) => return Err(error)
    }
}

fn bytes_to_u32(bytes: &[u8], endianness: &[u8;2])->u32{
    //!Converts 4 8bit unsigned integers into a single unsigned 32bit Integer
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
    //!Converts 2 8bit unsigned integers into a single unsigned 16bit Integer
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

fn save_image(raw_img: &[u8], output:&String, size:u16, orientation:u32)->Result<(), Error>{
    //!Sets image orientation, sizes it down and saves it in the output location
    //!The default value for the orientation should be 1, as that does not change it at all (horizontal)
    let loaded_img = load_from_memory_with_format(raw_img, image::ImageFormat::Jpeg);
    let mut img = match loaded_img{
        Ok(image) => image,
        Err(_) => return Err(Error::new(ErrorKind::Other, "Failed loading the Image"))
    };

    match orientation{
        /*
        1: rotate 0 degrees
        6: rotate 90 degrees
        3: rotate 180 degrees
        8: rotate 270 degrees 
        */
        6 => img = img.rotate90(),
        3 => img = img.rotate180(),
        8 => img = img.rotate270(),
        _ => {}
    }
    let size_factor:f32;
    if size != 0{
        size_factor = size as f32 / img.width() as f32;
    }
    else{
        size_factor = 1.0;
    }
    img = img.thumbnail((img.width() as f32*size_factor)as u32, (img.width() as f32*size_factor)as u32);
    match img.save_with_format(output, image::ImageFormat::Png){
        Ok(()) => {},
        Err(_) => {return Err(Error::new(ErrorKind::Other, "Failed saving the Image"))}
    }
    Ok(())
}


fn main() -> Result<(), Error>{
    let args = Args::parse();
    let input: String = args.file;
    let output: String = args.output;
    let size: u16 = args.size;
    if input.ends_with("CRW") || input.ends_with("crw"){
        crw::extract_thumb(&input, &output, size)?;
    }
    else if input.ends_with("CR2")|| input.ends_with("cr2"){
        cr2::extract_thumb(&input, &output, size)?;
    }
    else if input.ends_with("CR3")|| input.ends_with("cr3"){
        cr3::extract_thumb(&input, &output, size)?;
    }
    else{
        return Err(Error::new(ErrorKind::Unsupported, "Unknown File type, are you sure that this is a Canon RAW File?"))
    }
    Ok(())
}
