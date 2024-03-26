use clap::Parser;
use std::fs;
mod crw;
mod cr2;
mod cr3;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short,long)]
    file: String,

    #[arg(short,long)]
    output: String
}

fn read_file(file_path: &String) ->Vec<u8>{
    let data = fs::read(file_path).unwrap();
    return data;
}

fn main() {
    let args = Args::parse();
    let input = args.file;
    let output = args.output;
    /*if input.ends_with("CRW") || input.ends_with("crw"){
        crw::extract_thumb(&input, &output)
    }*/
    if input.ends_with("CR2")|| input.ends_with("cr2"){
        cr2::extract_thumb(&input, &output)
    }
    if input.ends_with("CR3")|| input.ends_with("cr3"){
        cr3::extract_thumb(&input, &output)
    }
}
