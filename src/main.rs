use std::env::args;
use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};
use lz4::block::decompress;

const MAGIC: i32 = 0x7332_6d6d;
const VER: i32 = 0x0000_0004;

struct Frame {
    size_raw: i32,
    size_com: i32,
}

fn unpack(fpath: &str) {
    let fpath = Path::new(fpath);

    let basename = fpath.file_stem().unwrap().to_str().unwrap();

    let mut f = File::open(fpath).unwrap();

    if MAGIC != f.read_i32::<LittleEndian>().unwrap() {
        panic!("Incorrect magic number");
    }

    if VER != f.read_i32::<LittleEndian>().unwrap() {
        panic!("Incorrect version number");
    }

    let info = Frame {
        size_com: f.read_i32::<LittleEndian>().unwrap(),
        size_raw: f.read_i32::<LittleEndian>().unwrap(),
    };

    let data = Frame {
        size_com: f.read_i32::<LittleEndian>().unwrap(),
        size_raw: f.read_i32::<LittleEndian>().unwrap(),
    };

    let mut buf_info_com = vec![0u8; info.size_com as usize];

    f.read_exact(&mut buf_info_com).unwrap();

    let buf_info_raw = decompress(&buf_info_com, Some(info.size_raw)).unwrap();

    let mut f_info = File::create(format(format_args!("{basename}_info.json"))).unwrap();

    f_info.write_all(&buf_info_raw).unwrap();

    let mut buf_data_com = vec![0u8; data.size_com as usize];

    f.read_exact(&mut buf_data_com).unwrap();

    let buf_data_raw = decompress(&buf_data_com, Some(data.size_raw)).unwrap();

    let mut f_data = File::create(format(format_args!("{basename}_data.json"))).unwrap();

    f_data.write_all(&buf_data_raw).unwrap();
}

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        2 => unpack(&args[1]),
        3 => println!("{args:?}"),
        _ => println!(
            "Usage:
    {cmd} <game.sav>
    {cmd} <info.json> <data.json>",
            cmd = args[0]
        ),
    }
}
