extern crate cc;
use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

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
        .out_dir(dst.join("lib"))
        .compile("libdeflate");

        let src = env::current_dir().unwrap().join("libdeflate");
        let include = dst.join("include");
        fs::create_dir_all(&include).unwrap();
        fs::copy(src.join("libdeflate.h"), dst.join("include/libdeflate.h")).unwrap();
        println!("cargo:root={}", dst.display());
        println!("cargo:include={}", dst.join("include").display());
}
