# rlibdeflate

Rust bindings to [libdeflate](https://github.com/ebiggers/libdeflate)

Libdeflate is a block-based compressor that requires supplying the the
whole input/output buffer up-front. It is a simple,
performance-focused compression library that is typically used
[BAM](https://samtools.github.io/hts-specs/SAMv1.pdf) files.


# Decompression Example

**Note**: This is simplified. High-perf implementations (e.g. genomic
          pipelines) would recycle the `decompressor` and buffers
          accordingly.

```rust
use crate::rlibdeflate;

use libdeflate::Decompressor;

fn decompress(deflate_data: &[u8], out_sz: usize) -> Vec<u8> {
    let mut decompressor = Decompressor::new();
    let mut ret = Vec::new();
    ret.resize(out_sz, 0);

    decompressor.decompress_deflate(&deflate_data, &mut ret).unwrap();

    return ret;
}
```


# Compression Example

**Note**: This is simplified. High-perf implementations (e.g. genomic
          pipelines) would recycle the `decompressor` and buffers
          accordingly.

```rust
use crate::rlibdeflate;

use libdeflate::Compressor;

fn compress(raw_data: &[u8]) -> Vec<u8> {
    let mut compressor = Compressor::with_default_compression_lvl();
    let max_out_size = compressor.compress_deflate_bound(raw_data.len());
    let mut out = Vec::new();
    out.resize(max_out_size, 0);
    let actual_out_size = compressor.compress_deflate(&raw_data, &mut out).unwrap();
    out.resize(actual_out_size, 0);

    return out;
}
```
