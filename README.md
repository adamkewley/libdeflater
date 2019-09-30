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
bib       111          3.0        5117           1730
book1     768          3.5        56648          16112
book2     610          3.2        34431          10749
geo       102          9.1        12301          1352
news      377          3.1        17814          5720
obj1      21           2.9        586            202
obj2      246          3.9        11738          3036
paper1    53           2.7        2181           800
paper2    82           3.0        4452           1469
pic       513          2.8        11343          4089
progc     39           2.5        1321           533
progl     71           2.8        2349           850
progp     49           2.4        1283           524
trans     93           2.5        2213           896
```

### Decompression

```
bench     size [KB]    speedup    flate2 [us]    libdeflate [us]
bib       111          2.3        371            165
book1     768          2.1        2894           1385
book2     610          2.3        2128           912
geo       102          1.7        507            295
news      377          2.1        1453           696
obj1      21           1.9        85             45
obj2      246          2.2        864            385
paper1    53           2.1        178            85
paper2    82           2.2        281            130
pic       513          2.9        1023           357
progc     39           2.0        130            64
progl     71           2.3        182            78
progp     49           2.2        123            56
trans     93           2.3        216            95
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
