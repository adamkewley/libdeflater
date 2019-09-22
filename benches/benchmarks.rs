#[macro_use]
extern crate criterion;
extern crate flate2;
extern crate libdeflate;
extern crate toml;

use criterion::{Criterion, BenchmarkId, Throughput};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::io::prelude::*;
use libdeflate::deflate::{Compressor, CompressionLvl};
use libdeflate::inflate::{Decompressor};
use std::fs;
use std::fs::{File};
use serde::{Deserialize};
use std::collections::{HashMap};

// Custom benchmarks can be specified in
// benches/custom-benchmarks.toml. They are benchmarks over data that
// isn't kept in the main repo.
#[derive(Deserialize)]
struct CustomBenchmark {
    path: String,
    summary: String,
    description: String,
}

// ENCODERS: Encoder assumptions are that an encoder may have state
// (constructed via ::new) and an ::encode method that receives data +
// output buffer to do a one-shot compression. Libdeflate cannot do
// streaming compression, so these benchmarks can only show one-shot
// perf.

// flate2
struct Flate2Encoder();

impl Flate2Encoder {
    fn new() -> Flate2Encoder {
        Flate2Encoder{}
    }

    fn encode(&mut self, data: &[u8], out: &mut Vec<u8>) {
        let mut e = GzEncoder::new(out, Compression::Default);
        e.write(data).unwrap();
        let _compressed_bytes = e.finish();
    }
}

struct Flate2Decoder();

impl Flate2Decoder {
    fn new() -> Flate2Decoder {
        Flate2Decoder{}
    }

    fn decode(&mut self, gz_data: &[u8], decomp_sz: usize, out: &mut Vec<u8>) {
        let mut d = GzDecoder::new(gz_data).unwrap();
        d.read(out).unwrap();
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
        out.resize(self.compressor.compress_gzip_bound(data.len()), 0);
        let actual = self.compressor.compress_gzip(data, out).unwrap();
        out.resize(actual, 0);
    }
}

struct LibdeflateDecoder {
    st: Decompressor,
}

impl LibdeflateDecoder {
    fn new() -> LibdeflateDecoder {
        LibdeflateDecoder { st: Decompressor::new() }
    }


    fn decode(&mut self, gz_data: &[u8], decomp_sz: usize, out: &mut Vec<u8>) {
        out.resize(decomp_sz, 0);
        let sz = self.st.decompress_gzip(gz_data, out).unwrap();
        out.resize(sz, 0);
    }
}

// Custom benchmarks are configured in
// `benches/custom-benches.toml`. It's done this way because the
// benchmarked data can't be kept in the repo and different devs will
// want to benchmark different corpuses of data to see if libdeflate
// is worth the plunge.
fn custom_benchmarks(b: &mut Criterion) {
    let cfg: HashMap<String, CustomBenchmark> = toml::from_str(&fs::read_to_string("benches/custom-benches.toml").unwrap()).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let mut flate2_enc = Flate2Encoder::new();
    let mut libdeflate_enc = LibdeflateEncoder::new();
    
    for k in cfg.keys() {        
        let entry = cfg.get(k).unwrap();
        let mut grp = b.benchmark_group(&entry.summary);
        
        let data = {
            let mut file = File::open(&entry.path).unwrap();
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            data
        };

        grp.throughput(Throughput::Elements(data.len() as u64));
        
        grp.bench_function("flate2", |b| b.iter(|| {
            flate2_enc.encode(&data, &mut buf);
            buf.resize(0, 0);
        }));

        grp.bench_function("libdeflate", |b| b.iter(|| {
            libdeflate_enc.encode(&data, &mut buf);
            buf.resize(0, 0);
        }));

        grp.finish();
    }
}

criterion_group!(benches, custom_benchmarks);
criterion_main!(benches);
