extern crate cc;

fn main() {
    cc::Build::new()
        .file("libdeflate/lib/aligned_malloc.c")
        .file("libdeflate/lib/deflate_decompress.c")
        .file("libdeflate/lib/arm/cpu_features.c")
        .file("libdeflate/lib/x86/cpu_features.c")
        .file("libdeflate/lib/deflate_compress.c")
        .file("libdeflate/lib/adler32.c")
        .file("libdeflate/lib/zlib_decompress.c")
        .file("libdeflate/lib/zlib_compress.c")
        .file("libdeflate/lib/crc32.c")
        .file("libdeflate/lib/gzip_decompress.c")
        .file("libdeflate/lib/gzip_compress.c")
        .include("libdeflate/")
        .include("libdeflate/lib/")
        .include("libdeflate/common/")
        .warnings(false)
        .compile("libdeflate");
}
