extern crate libdeflater;

use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use libdeflater::{Compressor, CompressionLvl, CompressionError, Decompressor, DecompressionError};



// decompression

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

fn read_fixture_zlib_with_bad_cmf_field() -> Vec<u8> {
    const BAD_CM: u8 = 9;  // RFC1950: val == 8 for deflate
    const BAD_CINFO: u8 = 8;  // RFC1950: vals > 7 disallowed
    const BAD_CMF: u8 = ((BAD_CM << 4) & 0xf0) | (BAD_CINFO & 0x0f);

    let mut data = read_fixture_zlib();

    data[0] = BAD_CMF;

    data
}

fn read_fixture_zlib_with_bad_adler32_checksum() -> Vec<u8> {
    let mut data = read_fixture_zlib();
    let adler32_start = data.len() - 4;

    data[adler32_start] = data[adler32_start] + 1;

    data
}

fn read_fixture_deflate() -> Vec<u8> {
    read_fixture("tests/hello.deflate")
}


// gz decompression

#[test]
fn test_calling_decompress_gzip_with_valid_args_works() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);

    decompressor.decompress_gzip(&content, &mut decompressed).unwrap();
}

#[test]
fn test_calling_decompress_gzip_with_undersized_outbuf_returns_insufficient_space() {
    let result = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressor.decompress_gzip(&content, &mut decompressed)
    };

    assert_eq!(result.unwrap_err(), DecompressionError::InsufficientSpace);
}

#[test]
fn test_calling_decompress_gzip_with_valid_buf_fills_buf_with_expected_content() {
    let decompressed = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);
        decompressor.decompress_gzip(&content, &mut decompressed).unwrap();
        decompressed
    };

    let expected_content = read_fixture_content();

    assert_eq!(decompressed, expected_content);
}

#[test]
fn test_calling_decompress_gzip_with_oversized_buf_returns_correct_size() {
    const OVERSIZED_FACTOR: usize = 2;

    let sz = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressed.resize(OVERSIZED_FACTOR*fixture_content_size(), 0);
        let sz = decompressor.decompress_gzip(&content, &mut decompressed).unwrap();
        sz
    };

    assert_eq!(sz, fixture_content_size());
}

#[test]
fn test_calling_decompress_gzip_with_bad_magic_num_returns_bad_data() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz_with_bad_magic_num();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);
    let result = decompressor.decompress_gzip(&content, &mut decompressed);

    assert_eq!(result.unwrap_err(), DecompressionError::BadData);
}

#[test]
fn test_calling_decompress_gzip_with_corrupted_crc32_returns_bad_data() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz_with_bad_crc32();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);
    let result = decompressor.decompress_gzip(&content, &mut decompressed);

    assert_eq!(result.unwrap_err(), DecompressionError::BadData);
}

#[test]
fn test_calling_decompress_gzip_with_corrupted_isize_returns_bad_data() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz_with_bad_isize();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);
    let result = decompressor.decompress_gzip(&content, &mut decompressed);

    assert_eq!(result.unwrap_err(), DecompressionError::BadData);
}


// zlib decompression

#[test]
fn test_calling_decompress_zlib_with_valid_args_works() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_zlib();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);

    decompressor.decompress_zlib(&content, &mut decompressed).unwrap();
}

#[test]
fn test_calling_decompress_zlib_with_undersized_outbuf_returns_insufficient_space() {
    let result = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib();
        let mut decompressed = Vec::with_capacity(0);

        decompressor.decompress_zlib(&content, &mut decompressed)
    };

    assert_eq!(result.unwrap_err(), DecompressionError::InsufficientSpace);
}

#[test]
fn test_calling_decompress_zlib_with_valid_buf_fills_buf_with_expected_content() {
    let decompressed_content = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);
        decompressor.decompress_zlib(&content, &mut decompressed).unwrap();
        decompressed
    };

    let expected_content = read_fixture_content();

    assert_eq!(decompressed_content, expected_content);
}

#[test]
fn test_calling_decompress_zlib_with_oversized_buf_returns_correct_size() {
    const OVERSIZED_FACTOR: usize = 2;

    let sz = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib();
        let mut decompressed = Vec::new();
        decompressed.resize(OVERSIZED_FACTOR*fixture_content_size(), 0);
        let sz = decompressor.decompress_zlib(&content, &mut decompressed).unwrap();
        sz
    };

    assert_eq!(sz, fixture_content_size());
}

#[test]
fn test_calling_decompress_zlib_with_bad_cmf_field_returns_bad_data() {
    let ret = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib_with_bad_cmf_field();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);

        decompressor.decompress_zlib(&content, &mut decompressed)
    };

    assert_eq!(ret.unwrap_err(), DecompressionError::BadData);
}

#[test]
fn test_calling_decompress_zlib_with_bad_adler32_checksum_returns_bad_data() {
    let ret = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib_with_bad_adler32_checksum();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);

        decompressor.decompress_zlib(&content, &mut decompressed)
    };

    assert_eq!(ret.unwrap_err(), DecompressionError::BadData);
}


// DEFLATE decompression

#[test]
fn test_calling_decompress_deflate_with_valid_args_works() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_deflate();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);

    decompressor.decompress_deflate(&content, &mut decompressed).unwrap();
}

#[test]
fn test_calling_decompress_deflate_with_undersized_outbuf_returns_insufficient_space() {
    let result = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_deflate();
        let mut decompressed = Vec::with_capacity(0);

        decompressor.decompress_deflate(&content, &mut decompressed)
    };

    assert_eq!(result.unwrap_err(), DecompressionError::InsufficientSpace);
}

#[test]
fn test_calling_decompress_deflate_with_valid_buf_fills_buf_with_expected_content() {
    let decompressed_content = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_deflate();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);
        decompressor.decompress_deflate(&content, &mut decompressed).unwrap();
        decompressed
    };

    let expected_content = read_fixture_content();

    assert_eq!(decompressed_content, expected_content);
}

#[test]
fn test_calling_decompress_deflate_with_oversized_buf_returns_correct_size() {
    const OVERSIZED_FACTOR: usize = 2;

    let sz = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_deflate();
        let mut decompressed = Vec::new();
        decompressed.resize(OVERSIZED_FACTOR*fixture_content_size(), 0);
        let sz = decompressor.decompress_deflate(&content, &mut decompressed).unwrap();
        sz
    };

    assert_eq!(sz, fixture_content_size());
}



// compression

#[test]
fn test_compressor_with_fastest_compression_lvl_calls_with_no_panics() {
    Compressor::new(CompressionLvl::fastest());
}

#[test]
fn test_compressor_with_default_compression_lvl_calls_with_no_panics() {
    Compressor::new(CompressionLvl::default());
}

#[test]
fn test_compressor_with_best_compression_lvl_calls_with_no_panics() {
    Compressor::new(CompressionLvl::best());
}

#[test]
fn test_compressor_with_compression_lvl_can_be_called_with_all_lvls_with_no_panics() {
    for lvl in CompressionLvl::iter() {
        Compressor::new(lvl);
    }
}


// compress DEFLATE

#[test]
fn test_compressor_compress_deflate_bound_returns_nonzero_when_given_nonzero_input() {
    let returned_bound = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        compressor.compress_deflate_bound(1)
    };

    assert_ne!(returned_bound, 0);
}

#[test]
fn test_compressor_compress_deflate_with_valid_args_does_not_panic() {
    let mut compressor = Compressor::new(CompressionLvl::default());
    let in_data = read_fixture_content();
    let mut out_data = Vec::new();
    let out_max_sz = compressor.compress_deflate_bound(in_data.len());
    out_data.resize(out_max_sz, 0);

    compressor.compress_deflate(&in_data, &mut out_data).unwrap();
}

#[test]
fn test_compressor_compress_deflate_with_undersized_buf_returns_insufficient_space() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        compressor.compress_deflate(&in_data, &mut out_data)
    };

    assert_eq!(ret.unwrap_err(), CompressionError::InsufficientSpace);
}

#[test]
fn test_compressor_compress_deflate_with_correct_args_returns_a_nonzero_size() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.compress_deflate_bound(in_data.len());
        out_data.resize(out_max_sz, 0);
        compressor.compress_deflate(&in_data, &mut out_data).unwrap()
    };

    assert_ne!(0, ret);
}

#[test]
fn test_compressor_compress_deflate_with_correct_args_resized_outbuf_to_return_val_produces_nonempty_nonzero_data() {
    // This is, give or take, essentially how users *should* use the
    // lib for raw DEFLATE compressions

    let compressed_data = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.compress_deflate_bound(in_data.len());
        out_data.resize(out_max_sz, 0);
        let sz = compressor.compress_deflate(&in_data, &mut out_data).unwrap();
        out_data.resize(sz, 0);
        out_data
    };

    assert_ne!(compressed_data.len(), 0);
    assert!(compressed_data.iter().any(|byte| *byte != 0x00));
}


// compress zlib

#[test]
fn test_compressor_compress_zlib_bound_returns_nonzero_when_given_nonzero_input() {
    let returned_bound = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        compressor.compress_zlib_bound(1)
    };

    assert_ne!(returned_bound, 0);
}

#[test]
fn test_compressor_compresss_zlib_with_valid_args_does_not_panic() {
    let mut compressor = Compressor::new(CompressionLvl::default());
    let in_data = read_fixture_content();
    let mut out_data = Vec::new();
    let out_max_sz = compressor.compress_zlib_bound(in_data.len());
    out_data.resize(out_max_sz, 0);

    compressor.compress_zlib(&in_data, &mut out_data).unwrap();
}

#[test]
fn test_compressor_compress_zlib_with_undersized_outbuf_returns_insufficient_space() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        compressor.compress_zlib(&in_data, &mut out_data)
    };

    assert_eq!(ret.unwrap_err(), CompressionError::InsufficientSpace);
}

#[test]
fn test_compressor_compress_zlib_with_correct_args_returns_nonzero_size() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.compress_zlib_bound(in_data.len());
        out_data.resize(out_max_sz, 0);

        compressor.compress_zlib(&in_data, &mut out_data).unwrap()
    };

    assert_ne!(ret, 0);
}

#[test]
fn test_compressor_compress_zlib_with_correct_args_resized_outbuf_to_return_val_produces_nonempty_nonzero_data() {
    // This is, give or take, essentially how users *should* use the
    // lib for zlib compressions

    let compressed_data = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.compress_zlib_bound(in_data.len());
        out_data.resize(out_max_sz, 0);
        let sz = compressor.compress_zlib(&in_data, &mut out_data).unwrap();
        out_data.resize(sz, 0);
        out_data
    };

    assert_ne!(compressed_data.len(), 0);
    assert!(compressed_data.iter().any(|byte| *byte != 0x00));
}


// compress gzip

#[test]
fn test_compressor_compress_gzip_bound_returns_nonzero_when_given_nonzero_input() {
    let returned_bound = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        compressor.compress_gzip_bound(1)
    };

    assert_ne!(returned_bound, 0);
}

#[test]
fn test_compressor_compresss_gzip_with_valid_args_does_not_panic() {
    let mut compressor = Compressor::new(CompressionLvl::default());
    let in_data = read_fixture_content();
    let mut out_data = Vec::new();
    let out_max_sz = compressor.compress_gzip_bound(in_data.len());
    out_data.resize(out_max_sz, 0);

    compressor.compress_gzip(&in_data, &mut out_data).unwrap();
}

#[test]
fn test_compressor_compress_gzip_with_undersized_outbuf_returns_insufficient_space() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        compressor.compress_gzip(&in_data, &mut out_data)
    };

    assert_eq!(ret.unwrap_err(), CompressionError::InsufficientSpace);
}

#[test]
fn test_compressor_compress_gzip_with_correct_args_returns_nonzero_size() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.compress_gzip_bound(in_data.len());
        out_data.resize(out_max_sz, 0);

        compressor.compress_gzip(&in_data, &mut out_data).unwrap()
    };

    assert_ne!(ret, 0);
}

#[test]
fn test_compressor_compress_gzip_with_correct_args_resized_outbuf_to_return_val_produces_nonempty_nonzero_data() {
    // This is, give or take, essentially how users *should* use the
    // lib for gzip compressions

    let compressed_data = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.compress_gzip_bound(in_data.len());
        out_data.resize(out_max_sz, 0);
        let sz = compressor.compress_gzip(&in_data, &mut out_data).unwrap();
        out_data.resize(sz, 0);
        out_data
    };

    assert_ne!(compressed_data.len(), 0);
    assert!(compressed_data.iter().any(|byte| *byte != 0x00));
}


// compress + decompress (full-cycle tests)

#[test]
fn test_compress_zlib_then_decompress_zlib_works_and_produces_the_same_input_data() {
    let input_data = read_fixture_content();
    
    let compression_buf = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let mut compression_buf = Vec::new();
        let compressed_max_sz = compressor.compress_zlib_bound(input_data.len());
        compression_buf.resize(compressed_max_sz, 0);
        let compressed_sz = compressor.compress_zlib(&input_data, &mut compression_buf).unwrap();
        compression_buf.resize(compressed_sz, 0);
        compression_buf
    };

    let decompressed_buf = {
        let mut decompressor = Decompressor::new();
        let mut decompressed_buf = Vec::new();
        decompressed_buf.resize(input_data.len(), 0);
        let decompressed_sz = decompressor.decompress_zlib(&compression_buf, &mut decompressed_buf).unwrap();

        assert_eq!(decompressed_sz, input_data.len());
        decompressed_buf
    };

    assert_eq!(input_data, decompressed_buf);
}

#[test]
fn test_compress_deflate_then_decompress_deflate_works_and_produces_the_same_input_data() {
    let input_data = read_fixture_content();
    
    let compression_buf = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let mut compression_buf = Vec::new();
        let compressed_max_sz = compressor.compress_deflate_bound(input_data.len());
        compression_buf.resize(compressed_max_sz, 0);
        let compressed_sz = compressor.compress_deflate(&input_data, &mut compression_buf).unwrap();
        compression_buf.resize(compressed_sz, 0);
        compression_buf
    };

    let decompressed_buf = {
        let mut decompressor = Decompressor::new();
        let mut decompressed_buf = Vec::new();
        decompressed_buf.resize(input_data.len(), 0);
        let decompressed_sz = decompressor.decompress_deflate(&compression_buf, &mut decompressed_buf).unwrap();

        assert_eq!(decompressed_sz, input_data.len());
        decompressed_buf
    };

    assert_eq!(input_data, decompressed_buf);
}

#[test]
fn test_compress_gzip_then_decompress_gzip_works_and_produces_the_same_input_data() {
    let input_data = read_fixture_content();
    
    let compression_buf = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let mut compression_buf = Vec::new();
        let compressed_max_sz = compressor.compress_gzip_bound(input_data.len());
        compression_buf.resize(compressed_max_sz, 0);
        let compressed_sz = compressor.compress_gzip(&input_data, &mut compression_buf).unwrap();
        compression_buf.resize(compressed_sz, 0);
        compression_buf
    };

    let decompressed_buf = {
        let mut decompressor = Decompressor::new();
        let mut decompressed_buf = Vec::new();
        decompressed_buf.resize(input_data.len(), 0);
        let decompressed_sz = decompressor.decompress_gzip(&compression_buf, &mut decompressed_buf).unwrap();

        assert_eq!(decompressed_sz, input_data.len());
        decompressed_buf
    };

    assert_eq!(input_data, decompressed_buf);
}
