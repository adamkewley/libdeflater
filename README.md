# libdeflater

[![Build Status](https://travis-ci.org/adamkewley/libdeflater.svg?branch=master)](https://travis-ci.org/adamkewley/libdeflater)
[![Crates.io](https://img.shields.io/crates/v/libdeflater.svg?maxAge=2592000)](https://crates.io/crates/libdeflater)
[![Documentation](https://docs.rs/libdeflater/badge.svg)](https://docs.rs/libdeflater)

Rust bindings to [libdeflate](https://github.com/ebiggers/libdeflate).
A high-performance library for working with gzip/zlib/deflate data.

```
libdeflater = "0.3.0"
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

Benchmark data is from both the [Calgary Corpus](https://en.wikipedia.org/wiki/Calgary_corpus), and the
[Canterbury Corpus](http://corpus.canterbury.ac.nz/resources/cantrbry.zip). The
benchmark tables below were made with this set of steps:

```bash
wget http://www.data-compression.info/files/corpora/largecalgarycorpus.zip
unzip -d bench_data largecalgarycorpus.zip
wget http://corpus.canterbury.ac.nz/resources/cantrbry.zip
unzip -d bench_data cantrbry.zip

# runs benchmarks against all files in `bench_data`
cargo bench
scripts/process-bench.rb encode
scripts/process-bench.rb decode
```

### Compression

Avg. speedup (on this corpus) is around 2-3x

```
bench           size [KB]    speedup    flate2 [us]    libdeflate [us]
alice29.txt     152          3.2        9883           3132
asyoulik.txt    125          3.0        7792           2572
bib             111          2.8        5380           1907
book1           768          3.2        55512          17588
book2           610          2.9        33751          11719
cp.html         24           2.0        571            280
fields.c        11           2.1        229            108
geo             102          8.4        12142          1441
grammar.lsp     3            1.7        64             38
kennedy.xls     1029         6.4        62130          9695
lcet10.txt      426          3.1        23727          7733
news            377          2.7        15814          5898
obj1            21           2.5        543            220
obj2            246          3.7        12202          3336
paper1          53           2.5        2149           865
paper2          82           2.7        4410           1613
paper3          46           2.5        2179           873
paper4          13           2.0        341            174
paper5          11           1.9        269            144
paper6          38           2.3        1350           584
pic             513          2.6        11364          4426
plrabn12.txt    481          3.4        38339          11266
progc           39           2.4        1401           590
progl           71           2.6        2301           897
progp           49           2.5        1322           531
ptt5            513          2.6        11379          4455
sum             38           3.7        1606           435
trans           93           2.5        2403           958
xargs.1         4            1.7        72             42
```

### Decompression

Avg. speedup (on this corpus) is around 2x.

```
bench           size [KB]    speedup    flate2 [us]    libdeflate [us]
alice29.txt     152          2.5        601            244
asyoulik.txt    125          2.2        519            234
bib             111          2.5        413            165
book1           768          2.2        3204           1478
book2           610          2.4        2405           987
cp.html         24           2.0        81             41
fields.c        11           2.1        35             17
geo             102          1.8        553            315
grammar.lsp     3            1.8        16             9
kennedy.xls     1029         2.1        2352           1105
lcet10.txt      426          2.5        1505           607
news            377          2.1        1570           738
obj1            21           1.9        94             50
obj2            246          2.3        971            431
paper1          53           2.3        205            90
paper2          82           2.4        341            143
paper3          46           2.1        192            90
paper4          13           1.7        54             32
paper5          11           1.9        52             28
paper6          38           2.1        140            68
pic             513          3.0        1140           386
plrabn12.txt    481          2.2        1973           880
progc           39           2.2        153            70
progl           71           2.5        214            86
progp           49           2.3        135            58
ptt5            513          3.0        1144           386
sum             38           2.1        147            71
trans           93           2.4        246            104
xargs.1         4            1.7        19             11
```

### Benchmark Notes

- All benchmarks are single-threaded

- IO/streaming overhead is not considered. The decompressed data is
  read into memory before performing the comparison

- Comparison made against `flate2` with no feature flags (i.e. `miniz`
  implementation). `flate2` was chosen because it's the most
  popular.

- Comparisons with other `flate2` backends are available on the
  `bench-flate2-miniz-oxide` and `bench-flate2-zlib` branches. The
  `zlib` backend is ~8 % faster on some of the corpus entries.

- Compression performed with default compression setting in both cases

- Corpus entries were compressed with `flate2` at default compression
  level
