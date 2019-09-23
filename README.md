# rlibdeflate

Rust bindings to [libdeflate](https://github.com/ebiggers/libdeflate)

Because libdeflate requires complete buffers up-front, it's unsuitable
for applications that rely on streaming compression (e.g. webservers,
applications that accept arbitrary sizes ofinput data). Rustaceans
looking for a general-purpose compressor should use something like
[flate2](https://github.com/alexcrichton/flate2-rs).

For applications that typically have all input data up front, and have
a mechanism for chunking large input datasets (e.g. genomic data, vcs
systems, specialized backends, game netcode compressors), libdeflate
might be worth considering because it has a simple API and is
typically faster than streaming compressors for small inputs (see
benchmarks).

# Decompression Example

TODO


# Benchmarks

- Benchmark data is from the [Calgary
  Corpus](https://en.wikipedia.org/wiki/Calgary_corpus), which has a
  decent range of input data types + sizes.

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


## Compression Benchmarks

- Compression performed with default compression setting in both cases

```
bench     size [KB]    flate2 [us]    libdeflate [us]    speedup
paper1    53           2141           813                2.6
progc     39           1364           534                2.6
trans     93           2463           893                2.8
obj1      21           561            206                2.7
pic       513          11699          4222               2.8
paper2    82           4200           1472               2.9
book1     768          52775          16020              3.3
progl     71           2437           852                2.9
geo       102          11651          1347               8.6
obj2      246          11674          3026               3.9
news      377          16108          5661               2.8
bib       111          5219           1818               2.9
progp     49           1430           528                2.7
book2     610          32777          10779              3.0
```

## Decompression Benchmarks

- Corpus entries were compressed with `flate2` at default compression
  level

```
bench     size [KB]    flate2 [us]    libdeflate [us]    speedup
paper1    53           321            88                 3.7
progc     39           234            66                 3.5
trans     93           450            102                4.4
obj1      21           146            53                 2.8
pic       513          2408           424                5.7
paper2    82           512            133                3.8
book1     768          5163           1399               3.7
progl     71           368            82                 4.5
geo       102          764            301                2.5
obj2      246          1633           440                3.7
news      377          2475           715                3.5
bib       111          711            183                3.9
progp     49           272            63                 4.3
book2     610          4086           1009               4.1
```
