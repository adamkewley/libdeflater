extern crate libdeflater;

use std::vec::Vec;
use std::mem::MaybeUninit;
use libdeflater::{Compressor, CompressionLvl};

fn main() {
    let str_to_compress = "hello\n";
    let str_bytes = str_to_compress.as_bytes().clone();

    let compressed_data = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let max_sz = compressor.gzip_compress_bound(str_bytes.len());
        let mut compressed_data = Vec::new();
        compressed_data.resize(max_sz, 0);
        let compressed_slice = compressor.gzip_compress(&str_bytes, compressed_data as &mut [u8]).unwrap();
        compressed_data.resize(compressed_slice.len(), 0);
        compressed_data
    };

    println!("compressed data: {:?}", compressed_data);
}
