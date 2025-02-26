extern crate libdeflater;

use libdeflater::Decompressor;
use std::fs::File;
use std::io::Read;
use std::str;
use std::vec::Vec;

fn main() {
    let gz_data = {
        let mut f = File::open("tests/hello.gz").unwrap();
        let mut content = Vec::new();
        f.read_to_end(&mut content).unwrap();
        content
    };

    assert!(
        !(gz_data.len() < 10),
        "gz data is too short (for magic bytes + footer"
    );

    // gzip RFC1952: a valid gzip file has an ISIZE field in the
    // footer, which is a little-endian u32 number representing the
    // decompressed size. This is ideal for libdeflate, which needs
    // preallocating the decompressed buffer.
    let isize = {
        let isize_start = gz_data.len() - 4;
        let isize_bytes = &gz_data[isize_start..];
        let mut ret: u32 = u32::from(isize_bytes[0]);
        ret |= u32::from(isize_bytes[1]) << 8;
        ret |= u32::from(isize_bytes[2]) << 16;
        ret |= u32::from(isize_bytes[3]) << 24;
        ret as usize
    };

    println!(
        "input data length = {}, expected output data length = {}",
        gz_data.len(),
        isize
    );

    let decompressed_data = {
        let mut decompressor = Decompressor::new();
        let mut outbuf = Vec::new();
        outbuf.resize(isize, 0);
        decompressor.gzip_decompress(&gz_data, &mut outbuf).unwrap();
        outbuf
    };

    println!(
        "output data = {:?}",
        str::from_utf8(&decompressed_data).unwrap()
    );
}
