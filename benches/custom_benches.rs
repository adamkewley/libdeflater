extern crate flate2;
extern crate libdeflater;
extern crate toml;

use std::io::prelude::*;
use std::fs;
use std::fs::{File};
use std::collections::{HashMap};
use serde::{Deserialize};

// benchmarking lib
use criterion::{Criterion};

// (de)compression libs
use flate2::{Compression, Compress, Decompress, FlushCompress, FlushDecompress};
use libdeflater::{Compressor, CompressionLvl, Decompressor};


// Custom benchmarks can be specified in
// benches/custom-benchmarks.toml. They are benchmarks over data that
// isn't kept in the main repo.
#[derive(Deserialize)]
struct CustomBenchmark {
    path: String,
}


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
        // because it essentially does exactly the same thing under
        // the hood
        unsafe {
            out.set_len(self.compressor.compress_zlib_bound(data.len()));
            let actual = self.compressor.compress_zlib(data, out).unwrap();
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
        // flate2's high-performance `Decompresss::decompress_vec`
        // method because it essentially does exactly the same thing
        // under the hood
        unsafe {
            out.set_len(decomp_sz);
            let sz = self.st.zlib_decompress(zlib_data, out).unwrap();
            out.set_len(sz);
        }
    }
}

// Custom benchmarks are configured in
// `benches/custom-benches.toml`. It's done this way because the
// benchmarked data can't be kept in the repo and different devs will
// want to benchmark different corpuses of data to see if libdeflate
// is suitable for their needs
pub fn run_custom_benches(b: &mut Criterion) {
    // flate2's low-level API will only handle the vector length, not
    // capacity, and flate2 doesn't have a "bounds" method so that
    // callers can pre-size the buffer for it, so this hack just makes
    // sure we have a buffer big enough to accomodate the corpus.
    const BUF_CAP_BIG_ENOUGH_TO_FIT_DATA: usize = 1<<20;

    let cfg: HashMap<String, CustomBenchmark> =
        toml::from_str(&fs::read_to_string("benches/custom-benches.toml").unwrap()).unwrap();

    let mut buf: Vec<u8> = Vec::with_capacity(BUF_CAP_BIG_ENOUGH_TO_FIT_DATA);
    let mut flate2_encoder = Flate2Encoder::new();
    let mut libdeflate_encoder = LibdeflateEncoder::new();
    let mut flate2_decoder = Flate2Decoder::new();
    let mut libdeflate_decoder = LibdeflateDecoder::new();

    for k in cfg.keys() {
        let entry = cfg.get(k).unwrap();
        let mut grp = b.benchmark_group(k);

        // these are quite big compressions
        grp.sample_size(20);

        let raw_data = {
            let mut file = File::open(&entry.path).unwrap();
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            data
        };

        grp.bench_function("flate2_encode", |b| b.iter(|| {
            buf.clear();
            flate2_encoder.encode(&raw_data, &mut buf);
        }));

        grp.bench_function("libdeflate_encode", |b| b.iter(|| {
            buf.clear();
            libdeflate_encoder.encode(&raw_data, &mut buf);
        }));

        let compressed_data = {
            let mut buf = Vec::with_capacity(BUF_CAP_BIG_ENOUGH_TO_FIT_DATA);
            Flate2Encoder::new().encode(&raw_data, &mut buf);
            buf
        };

        grp.bench_function("flate2_decode", |b| b.iter(|| {
            buf.clear();
            flate2_decoder.decode(&compressed_data, raw_data.len(),  &mut buf);
        }));

        grp.bench_function("libdeflate_decode", |b| b.iter(|| {
            buf.clear();
            libdeflate_decoder.decode(&compressed_data, raw_data.len(),  &mut buf);
        }));

        grp.finish();
    }
}
