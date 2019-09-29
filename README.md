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
bench     size [KB]    speedup    flate2 [us]    libdeflate [us]
bib       111          3.2        5310           1678
book1     768          3.4        52818          15691
book2     610          3.1        31973          10447
geo       102          8.7        11447          1316
news      377          2.8        16156          5699
obj1      21           2.7        539            203
obj2      246          3.8        11447          3011
paper1    53           2.7        2206           824
paper2    82           2.9        4298           1501
pic       513          2.8        11438          4082
progc     39           2.6        1426           548
progl     71           2.9        2444           838
progp     49           3.0        1548           524
trans     93           2.8        2432           882
```

### Decompression

```
bench     size [KB]    speedup    flate2 [us]    libdeflate [us]
bib       111          3.9        662            170
book1     768          3.7        5130           1390
book2     610          4.2        3822           921
geo       102          2.6        764            293
news      377          3.5        2529           714
obj1      21           2.9        136            47
obj2      246          3.7        1485           396
paper1    53           4.0        341            86
paper2    82           3.8        508            134
pic       513          5.7        2248           393
progc     39           3.6        244            68
progl     71           4.5        376            84
progp     49           4.3        262            61
trans     93           4.4        448            101
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
