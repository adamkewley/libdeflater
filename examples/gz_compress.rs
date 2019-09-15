use libdeflate;

use std::vec::Vec;
use libdeflate::deflate::{Compressor, CompressionLvl};

fn main() {
    let str_to_compress = "hello\n";
    let str_bytes = str_to_compress.as_bytes().clone();

    let compressed_data = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let max_sz = compressor.compress_gzip_bound(str_bytes.len());
        let mut compressed_data = Vec::new();
        compressed_data.resize(max_sz, 0);
        let actual_sz = compressor.compress_gzip(&str_bytes, &mut compressed_data).unwrap();
        compressed_data.resize(actual_sz, 0);
        compressed_data
    };

    println!("compressed data: {:?}", compressed_data);
}
