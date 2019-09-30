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
bib       111          2.9        4908           1690
book1     768          3.2        50381          15896
book2     610          2.9        30596          10504
geo       102          8.8        11833          1346
news      377          2.7        15130          5702
obj1      21           2.4        489            201
obj2      246          3.7        10887          2954
paper1    53           2.5        1969           794
paper2    82           2.8        3988           1438
pic       513          2.5        10408          4092
progc     39           2.4        1320           542
progl     71           2.6        2365           894
progp     49           2.5        1314           520
trans     93           2.6        2241           877
```

### Decompression

```
bench     size [KB]    speedup    flate2 [us]    libdeflate [us]
bib       111          2.3        381            167
book1     768          2.2        2989           1351
book2     610          2.5        2192           893
geo       102          1.8        542            306
news      377          2.2        1522           699
obj1      21           1.8        85             48
obj2      246          2.2        865            395
paper1    53           2.2        191            86
paper2    82           2.3        296            128
pic       513          2.8        1034           366
progc     39           2.2        147            68
progl     71           2.5        200            79
progp     49           2.4        134            56
trans     93           2.4        228            97
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
