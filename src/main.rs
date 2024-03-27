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
