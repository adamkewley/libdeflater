# libdeflater

[![Build Status](https://travis-ci.org/adamkewley/libdeflater.svg?branch=master)](https://travis-ci.org/adamkewley/libdeflater)
[![Crates.io](https://img.shields.io/crates/v/libdeflater.svg?maxAge=2592000)](https://crates.io/crates/libdeflater)
[![Documentation](https://docs.rs/libdeflater/badge.svg)](https://docs.rs/libdeflater)

Rust bindings to [libdeflate](https://github.com/ebiggers/libdeflate).
A high-performance library for working with gzip/zlib/deflate data.

```
libdeflater = "0.1.2"
```

**Warning**: libdeflate is for *specialized* use-cases. You should
             use something like [flate2](https://github.com/alexcrichton/flate2-rs)
             if you want a general-purpose deflate library.

libdeflate is optimal in applications that have all input data up and
have a mechanism for chunking large input datasets (e.g. genomic
[bam](https://samtools.github.io/hts-specs/SAMv1.pdf) files, some
object stores, specialized backends, game netcode packets). It has a
much simpler API than [zlib](https://www.zlib.net/manual.html) but
can't stream data.


# Examples

Example source [here](examples). To run the examples:

```bash
cargo run --example gz_compress.rs
cargo run --example gz_decompress.rs
```


# Benchmarks

Benchmark data is from the [Calgary Corpus](https://en.wikipedia.org/wiki/Calgary_corpus),
which has a decent range of input data types + sizes. See benchmark notes below for more
details. The benchmark tables below were made with this set of steps:

```bash
wget http://www.data-compression.info/files/corpora/largecalgarycorpus.zip
mkdir bench_data
unzip -d bench_data largecalgarycorpus.zip
cargo bench
scripts/process-bench.rb encode
scripts/process-bench.rb decode
```

### Compression

```
bench     size [KB]    speedup    flate2 [us]    libdeflate [us]
bib       111          2.9        4956           1711
book1     768          3.1        51660          16686
book2     610          2.9        30993          10684
geo       102          8.4        11297          1340
news      377          2.7        15172          5693
obj1      21           2.4        497            207
obj2      246          3.7        11207          3043
paper1    53           2.5        1998           801
paper2    82           2.8        4057           1463
pic       513          2.5        10564          4144
progc     39           2.4        1280           534
progl     71           2.7        2290           849
progp     49           2.5        1313           525
trans     93           2.6        2305           894
```

### Decompression

```
bench     size [KB]    speedup    flate2 [us]    libdeflate [us]
bib       111          2.5        422            170
book1     768          2.5        3547           1434
book2     610          2.6        2522           956
geo       102          1.8        551            302
news      377          2.3        1669           718
obj1      21           1.9        90             48
obj2      246          2.4        959            404
paper1    53           2.3        206            89
paper2    82           2.5        333            135
pic       513          3.0        1157           391
progc     39           2.3        150            66
progl     71           2.6        215            84
progp     49           2.5        153            62
trans     93           2.4        250            103
```

### Benchmark Notes

- Benchmark data is from the [Calgary Corpus](https://en.wikipedia.org/wiki/Calgary_corpus),
  which has a decent range of input data types + sizes.

- Benchmarks were ran by unpacking the corpus into `bench_data` in
  this repo then running `cargo bench` which, in turn, runs
  [this](benches/custom_benches.rs) custom benchmark suite configured
  with [this](benches/custom-benches.toml) config.

- The results were aggregated with [this](scripts/process-bench.rb)
  ruby script that extracts `Mean.point_estimate` from `Criterion`'s
  `estimates.json`

- All benchmarks are single-threaded

- Comparison made against `flate2` with no feature flags (i.e. `miniz`
  implementation). `flate2` was chosen because it's the most popular.

- Compression performed with default compression setting in both cases

- Corpus entries were compressed with `flate2` at default compression
  level
