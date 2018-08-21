extern crate byteorder;
extern crate lz4;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};
use lz4::block::decompress;

const MAGIC: i32 = 0x73326d6d;
const VER: i32 = 0x00000004;

struct Frame {
    size_raw: i32,
    size_com: i32,
}

fn unpack(fpath: &String) {
    let fpath = Path::new(fpath);

    let basename = fpath.file_stem().unwrap().to_str().unwrap();

    let mut f = File::open(fpath).expect("Unable to open file");

    if MAGIC
        != f.read_i32::<LittleEndian>()
            .expect("Fail to check magic number")
    {
        panic!("Incorrect magic number");
    }

    if VER != f
        .read_i32::<LittleEndian>()
        .expect("Fail to check version number")
    {
        panic!("Incorrect version number");
    }

    let info = Frame {
        size_com: f
            .read_i32::<LittleEndian>()
            .expect("Unable to read encoded size"),
        size_raw: f
            .read_i32::<LittleEndian>()
            .expect("Unable to read unencoded size"),
    };

    let data = Frame {
        size_com: f
            .read_i32::<LittleEndian>()
            .expect("Unable to read encoded size"),
        size_raw: f
            .read_i32::<LittleEndian>()
            .expect("Unable to read unencoded size"),
    };

    let mut buf_info_com = vec![0u8; info.size_com as usize];

    f.read(&mut buf_info_com).expect("Unable to read info");

    let buf_info_raw =
        decompress(&buf_info_com, Some(info.size_raw)).expect("Unable to decompress info");

    let mut f_info = File::create(fmt::format(format_args!("{}_info.json", basename)))
        .expect("Unable to create info");

    f_info.write(&buf_info_raw).expect("Unable to write info");

    let mut buf_data_com = vec![0u8; data.size_com as usize];

    f.read(&mut buf_data_com).expect("Unable to read data");

    let buf_data_raw =
        decompress(&buf_data_com, Some(data.size_raw)).expect("Unable to decompress data");

    let mut f_data = File::create(fmt::format(format_args!("{}_data.json", basename)))
        .expect("Unable to create data");

    f_data.write(&buf_data_raw).expect("Unable to write data");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => unpack(&args[1]),
        3 => println!("{:?}", args),
        _ => println!(
            "Usage:
    {cmd} <game.sav>
    {cmd} <info.json> <data.json>",
            cmd = args[0]
        ),
    }
}
