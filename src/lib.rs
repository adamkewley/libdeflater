extern crate libc;

use libc::{c_int, size_t};

pub enum LibdeflateCompressor {}

#[link(name = ":libdeflate.so.0")]
extern {
    pub fn libdeflate_alloc_compressor(compression_level: c_int) -> *mut LibdeflateCompressor;
    pub fn libdeflate_free_compressor(compressor: *mut LibdeflateCompressor) -> ();
    pub fn libdeflate_deflate_compress(compressor: *mut LibdeflateCompressor,
                                       in_buf: *const u8,
                                       in_nbytes: size_t,
                                       out_buf: *mut u8,
                                       out_nbytes_avail: size_t) -> size_t;
}
