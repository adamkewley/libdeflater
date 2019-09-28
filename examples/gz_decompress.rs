extern crate libdeflater;

use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use std::str;
use libdeflater::Decompressor;

fn main() {
    let gz_data = {
        let mut f = File::open("tests/hello.gz").unwrap();
        let mut content = Vec::new();
        f.read_to_end(&mut content).unwrap();
        content
    };

    if gz_data.len() < 10 {
        panic!("gz data is too short (for magic bytes + footer");
    }

    // gzip RFC1952: a valid gzip file has an ISIZE field in the
    // footer, which is a little-endian u32 number representing the
    // decompressed size. This is ideal for libdeflate, which needs
    // preallocating the decompressed buffer.
    let isize = {
        let isize_start = gz_data.len() - 4;
        let isize_bytes = &gz_data[isize_start..];
        let mut ret: u32 = isize_bytes[0] as u32;
        ret |= (isize_bytes[1] as u32) << 8;
        ret |= (isize_bytes[2] as u32) << 16;
        ret |= (isize_bytes[3] as u32) << 26;
        ret as usize
    };

    println!("input data length = {}, expected output data length = {}", gz_data.len(), isize);

    let decompressed_data = {
        let mut decompressor = Decompressor::new();
        let mut outbuf = Vec::new();
        outbuf.resize(isize, 0);
        decompressor.decompress_gzip(&gz_data, &mut outbuf).unwrap();
        outbuf
    };

    println!("output data = {:?}", str::from_utf8(&decompressed_data).unwrap());
}
