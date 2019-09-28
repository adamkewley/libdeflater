# libdeflater

[![Build Status](https://travis-ci.org/adamkewley/libdeflater.svg?branch=master)](https://travis-ci.org/adamkewley/libdeflater)
[![Crates.io](https://img.shields.io/crates/v/libdeflater.svg?maxAge=2592000)](https://crates.io/crates/libdeflater)
[![Documentation](https://docs.rs/libdeflater/badge.svg)](https://docs.rs/libdeflater)

Rust bindings to [libdeflate](https://github.com/ebiggers/libdeflate).
A high-performance library for working with gzip/zlib/deflate data.

```
libdeflater = "0.1.0"
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
bench     size [KB]    flate2 [us]    libdeflate [us]    speedup
bib       111          5164           1671               3.1
book1     768          56103          17216              3.3
book2     610          32104          10401              3.1
geo       102          11517          1313               8.8
news      377          15854          5548               2.9
obj1      21           543            200                2.7
obj2      246          12421          3161               3.9
paper1    53           2092           778                2.7
paper2    82           4176           1425               2.9
pic       513          12307          4337               2.8
progc     39           1446           559                2.6
progl     71           2654           891                3.0
progp     49           1416           513                2.8
trans     93           2424           875                2.8
```

### Decompression

```
bench     size [KB]    flate2 [us]    libdeflate [us]    speedup
bib       111          667            167                4.0
book1     768          5590           1500               3.7
book2     610          3864           927                4.2
geo       102          822            322                2.6
news      377          2483           702                3.5
obj1      21           136            47                 2.9
obj2      246          1593           426                3.7
paper1    53           319            87                 3.7
paper2    82           508            132                3.9
pic       513          2420           415                5.8
progc     39           248            70                 3.5
progl     71           398            88                 4.5
progp     49           250            58                 4.3
trans     93           447            101                4.4
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
