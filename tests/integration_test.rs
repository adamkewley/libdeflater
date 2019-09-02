extern crate libdeflate;

use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use libdeflate::inflate::{Decompressor, Error};

#[test]
fn test_can_create_new_decompressor() {
    Decompressor::new();
}

fn read_fixture(pth: &str) -> Vec<u8> {
    let mut f = File::open(pth).unwrap();
    let mut content = Vec::new();
    f.read_to_end(&mut content).unwrap();
    content
}

fn read_fixture_content() -> Vec<u8> {
    read_fixture("tests/hello")
}

fn fixture_content_size() -> usize {
    read_fixture_content().len()
}

fn read_fixture_gz() -> Vec<u8> {
    read_fixture("tests/hello.gz")
}

fn read_fixture_gz_with_bad_magic_num() -> Vec<u8> {
    let mut data = read_fixture_gz();
    
    // beats having to have an extra fixture file
    data[0] = 0x00;

    data
}

fn read_fixture_gz_with_bad_crc32() -> Vec<u8> {
    let mut data = read_fixture_gz();
    let crc32_start = data.len()-8;
    
    // beats having to have an extra fixture file
    data[crc32_start] = data[crc32_start] + 1;

    data
}

fn read_fixture_gz_with_bad_isize() -> Vec<u8> {
    let mut data = read_fixture_gz();
    let isize_start = data.len()-4;
    
    // beats having to have an extra fixture file
    data[isize_start] = data[isize_start] + 1;

    data
}

fn read_fixture_zlib() -> Vec<u8> {
    read_fixture("tests/hello.zz")
}

#[test]
fn test_calling_decompress_gz_with_valid_args_works() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);

    decompressor.decompress_gz(&content, &mut decompressed).unwrap();
}

#[test]
fn test_calling_decompress_gz_with_undersized_outbuf_returns_insufficient_space() {
    let result = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressor.decompress_gz(&content, &mut decompressed)
    };
    
    assert_eq!(result.unwrap_err(), Error::InsufficientSpace);
}

#[test]
fn test_calling_decompress_gz_with_valid_buf_fills_buf_with_expected_content() {
    let decompressed = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);
        decompressor.decompress_gz(&content, &mut decompressed).unwrap();
        decompressed
    };

    let expected_content = read_fixture_content();

    assert_eq!(decompressed, expected_content);
}

#[test]
fn test_calling_decompress_gz_with_oversized_buf_returns_correct_size() {
    const OVERSIZED_FACTOR: usize = 2;
    
    let sz = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressed.resize(OVERSIZED_FACTOR*fixture_content_size(), 0);
        let sz = decompressor.decompress_gz(&content, &mut decompressed).unwrap();
        sz
    };

    assert_eq!(sz, fixture_content_size());
}

#[test]
fn test_calling_decompress_gz_with_bad_magic_num_returns_bad_data() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz_with_bad_magic_num();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);
    let result = decompressor.decompress_gz(&content, &mut decompressed);

    assert_eq!(result.unwrap_err(), Error::BadData);
}

#[test]
fn test_calling_decompress_gz_with_corrupted_crc32_returns_bad_data() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz_with_bad_crc32();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);
    let result = decompressor.decompress_gz(&content, &mut decompressed);

    assert_eq!(result.unwrap_err(), Error::BadData);
}

#[test]
fn test_calling_decompress_gz_with_corrupted_isize_returns_bad_data() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz_with_bad_isize();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);
    let result = decompressor.decompress_gz(&content, &mut decompressed);

    assert_eq!(result.unwrap_err(), Error::BadData);
}

#[test]
fn test_calling_decompress_zlib_with_valid_args_works() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_zlib();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);

    decompressor.decompress_zlib(&content, &mut decompressed).unwrap();
}
