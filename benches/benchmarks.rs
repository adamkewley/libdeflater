#[macro_use]
extern crate criterion;
extern crate flate2;
extern crate libdeflate;

use criterion::Criterion;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use libdeflate::deflate::{Compressor};

fn flate2_encode() {
    let mut e = GzEncoder::new(Vec::new(), Compression::Default);
    e.write(b"foo");
    let compressed_bytes = e.finish();
}

fn libdeflate_encode() {
    let data = b"foo";
    let mut c = Compressor::with_default_compression_lvl();
    let mut out = Vec::new();
    out.resize(c.compress_gzip_bound(data.len()), 0);
    let actual = c.compress_gzip(data, &mut out).unwrap();
    out.resize(actual, 0);
}

fn bench_flate2(b: &mut Criterion) {
    b.bench_function("flate2_gz_encode", |b| b.iter(|| flate2_encode()));
    b.bench_function("libdeflate_gz_encode", |b| b.iter(|| libdeflate_encode()));
    
}

criterion_group!(benches, bench_flate2);
criterion_main!(benches);
