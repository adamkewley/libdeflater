extern crate flate2;
extern crate libdeflater;

use std::io::prelude::*;
use std::fs;
use std::fs::{File};
use std::path::Path;

// benchmarking lib
use criterion::{Criterion, black_box};

// (de)compression libs
use flate2::{Compression, Compress, Decompress, FlushCompress, FlushDecompress};
use libdeflater::{Compressor, CompressionLvl, Decompressor};


// flate2
struct Flate2Encoder {
    compress: Compress,
}

impl Flate2Encoder {
    fn new() -> Flate2Encoder {
        Flate2Encoder{ compress: Compress::new(Compression::default(), true) }
    }

    fn encode(&mut self, data: &[u8], out: &mut Vec<u8>) {
        self.compress.compress_vec(data, out, FlushCompress::Finish).unwrap();
        self.compress.reset();
    }
}

struct Flate2Decoder {
    decompress: Decompress,
}

impl Flate2Decoder {
    fn new() -> Flate2Decoder {
        Flate2Decoder { decompress: Decompress::new(true) }
    }

    fn decode(&mut self, compressed_data: &[u8], _decomp_sz: usize, out: &mut Vec<u8>) {
        self.decompress.decompress_vec(compressed_data, out, FlushDecompress::Finish).unwrap();
        self.decompress.reset(true);
    }
}

// libdeflate
struct LibdeflateEncoder {
    compressor: Compressor,
}

impl LibdeflateEncoder {
    fn new() -> LibdeflateEncoder {
        LibdeflateEncoder {
            compressor: Compressor::new(CompressionLvl::default()),
        }
    }

    fn encode(&mut self, data: &[u8], out: &mut Vec<u8>) {
        // This unsafe is fair for this particular comparison with
        // flate2's high-performance `Compress::compress_vec` method
        // because the flate2 method is essentially doing exactly the
        // same thing under the hood
        unsafe {
            out.set_len(self.compressor.zlib_compress_bound(data.len()));
            let actual = self.compressor.zlib_compress(data, out).unwrap();
            out.set_len(actual);
        }
    }
}

struct LibdeflateDecoder {
    st: Decompressor,
}

impl LibdeflateDecoder {
    fn new() -> LibdeflateDecoder {
        LibdeflateDecoder { st: Decompressor::new() }
    }


    fn decode(&mut self, zlib_data: &[u8], decomp_sz: usize, out: &mut Vec<u8>) {
        // This unsafe is fair for this particular comparison with
        // flate2's high-performance `Decompress::compress_vec` method
        // because the flate2 method is essentially doing exactly the
        // same thing under the hood
        unsafe {
            out.set_len(decomp_sz);
            let sz = self.st.zlib_decompress(zlib_data, out).unwrap();
            out.set_len(sz);
        }
    }
}

// Custom benchmarks exercise each decompressed file in the
// `bench_data` folder (apart from README.md). This allows developers
// to test what performance difference they're likely to see when
// comparing libdeflate to flate2
pub fn run_custom_benches(b: &mut Criterion) {
    let (entries, biggest_entry) = {
        let mut biggest: u64 = 0;
        let mut entries = Vec::new();

        let path = Path::new("bench_data");
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let pth = entry.path();
            let sz = entry.metadata().unwrap().len();
            let filename = pth.file_name().unwrap().to_str().unwrap();

            if entry.metadata().unwrap().is_file() && !filename.contains("README.md") {
                entries.push(pth);
                biggest = if sz > biggest { sz } else { biggest }
            }
        }

        (entries, biggest as usize)
    };

    // lets (hackily) assume a 100-byte compression container
    // overhead.
    let buf_big_enough_to_fit_data = biggest_entry + 100;

    let mut buf: Vec<u8> = Vec::with_capacity(buf_big_enough_to_fit_data);
    let mut flate2_encoder = Flate2Encoder::new();
    let mut libdeflate_encoder = LibdeflateEncoder::new();
    let mut flate2_decoder = Flate2Decoder::new();
    let mut libdeflate_decoder = LibdeflateDecoder::new();

    for pth in entries {
        let k = pth.file_name().unwrap().to_str().unwrap();
        let mut grp = b.benchmark_group(k);

        // these are quite big compressions
        grp.sample_size(20);

        let raw_data = {
            let mut file = File::open(&pth).unwrap();
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            data
        };

        grp.bench_function("flate2_encode", |b| b.iter(|| {
            buf.clear();
            flate2_encoder.encode(black_box(&raw_data), black_box(&mut buf));
        }));

        grp.bench_function("libdeflate_encode", |b| b.iter(|| {
            buf.clear();
            libdeflate_encoder.encode(black_box(&raw_data), black_box(&mut buf));
        }));

        let compressed_data = {
            let mut buf = Vec::with_capacity(buf_big_enough_to_fit_data);
            Flate2Encoder::new().encode(&raw_data, &mut buf);
            buf
        };

        grp.bench_function("flate2_decode", |b| b.iter(|| {
            buf.clear();
            flate2_decoder.decode(black_box(&compressed_data), black_box(raw_data.len()),  black_box(&mut buf));
        }));

        grp.bench_function("libdeflate_decode", |b| b.iter(|| {
            buf.clear();
            libdeflate_decoder.decode(black_box(&compressed_data), black_box(raw_data.len()),  black_box(&mut buf));
        }));

        grp.finish();
    }
}
