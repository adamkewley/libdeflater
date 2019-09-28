mod libdeflate_sys;

use crate::libdeflate_sys::{libdeflate_decompressor,
                            libdeflate_alloc_decompressor,
                            libdeflate_free_decompressor,
                            libdeflate_gzip_decompress,
                            libdeflate_zlib_decompress,
                            libdeflate_deflate_decompress,
                            libdeflate_result,
                            libdeflate_result_LIBDEFLATE_SUCCESS,
                            libdeflate_result_LIBDEFLATE_BAD_DATA,
                            libdeflate_result_LIBDEFLATE_INSUFFICIENT_SPACE,
                            libdeflate_compressor,
                            libdeflate_alloc_compressor,
                            libdeflate_deflate_compress_bound,
                            libdeflate_deflate_compress,
                            libdeflate_zlib_compress_bound,
                            libdeflate_zlib_compress,
                            libdeflate_gzip_compress_bound,
                            libdeflate_gzip_compress,
                            libdeflate_free_compressor};

pub struct Decompressor {
    p: *mut libdeflate_decompressor,
}

#[derive(Debug, PartialEq)]
pub enum DecompressionError {
    BadData,
    InsufficientSpace,
}

type DecompressionResult<T> = std::result::Result<T, DecompressionError>;

#[allow(non_upper_case_globals)]
impl Decompressor {

    pub fn new() -> Decompressor {
        unsafe {
            let ptr = libdeflate_alloc_decompressor();
            if !ptr.is_null() {
                Decompressor{ p: ptr }
            } else {
                panic!("libdeflate_alloc_decompressor returned NULL: out of memory");
            }
        }
    }

    pub fn decompress_gzip(&mut self,
                           gz_data: &[u8],
                           out: &mut [u8]) -> DecompressionResult<usize> {
        unsafe {
            let mut out_nbytes = 0;
            let in_ptr = gz_data.as_ptr() as *const std::ffi::c_void;
            let out_ptr = out.as_mut_ptr() as *mut std::ffi::c_void;
            let ret: libdeflate_result =
                libdeflate_gzip_decompress(self.p,
                                           in_ptr,
                                           gz_data.len(),
                                           out_ptr,
                                           out.len(),
                                           &mut out_nbytes);
            match ret {
                libdeflate_result_LIBDEFLATE_SUCCESS => {
                    Ok(out_nbytes)
                },
                libdeflate_result_LIBDEFLATE_BAD_DATA => {
                    Err(DecompressionError::BadData)
                },
                libdeflate_result_LIBDEFLATE_INSUFFICIENT_SPACE => {
                    Err(DecompressionError::InsufficientSpace)
                },
                _ => {
                    panic!("libdeflate_gzip_decompress returned an unknown error type: this is an internal bug that **must** be fixed");
                }
            }
        }
    }

    pub fn decompress_zlib(&mut self,
                           zlib_data: &[u8],
                           out: &mut [u8]) -> DecompressionResult<usize> {
        unsafe {
            let mut out_nbytes = 0;
            let in_ptr = zlib_data.as_ptr() as *const std::ffi::c_void;
            let out_ptr = out.as_mut_ptr() as *mut std::ffi::c_void;
            let ret: libdeflate_result =
                libdeflate_zlib_decompress(self.p,
                                           in_ptr,
                                           zlib_data.len(),
                                           out_ptr,
                                           out.len(),
                                           &mut out_nbytes);

            match ret {
                libdeflate_result_LIBDEFLATE_SUCCESS => {
                    Ok(out_nbytes)
                },
                libdeflate_result_LIBDEFLATE_BAD_DATA => {
                    Err(DecompressionError::BadData)
                },
                libdeflate_result_LIBDEFLATE_INSUFFICIENT_SPACE => {
                    Err(DecompressionError::InsufficientSpace)
                },
                _ => {
                    panic!("libdeflate_zlib_decompress returned an unknown error type: this is an internal bug that **must** be fixed");
                }
            }
        }
    }

    pub fn decompress_deflate(&mut self,
                              deflate_data: &[u8],
                              out: &mut [u8]) -> DecompressionResult<usize> {
        unsafe {
            let mut out_nbytes = 0;
            let in_ptr = deflate_data.as_ptr() as *const std::ffi::c_void;
            let out_ptr = out.as_mut_ptr() as *mut std::ffi::c_void;
            let ret: libdeflate_result =
                libdeflate_deflate_decompress(self.p,
                                              in_ptr,
                                              deflate_data.len(),
                                              out_ptr,
                                              out.len(),
                                              &mut out_nbytes);

            match ret {
                libdeflate_result_LIBDEFLATE_SUCCESS => {
                    Ok(out_nbytes)
                },
                libdeflate_result_LIBDEFLATE_BAD_DATA => {
                    Err(DecompressionError::BadData)
                },
                libdeflate_result_LIBDEFLATE_INSUFFICIENT_SPACE => {
                    Err(DecompressionError::InsufficientSpace)
                },
                _ => {
                    panic!("libdeflate_zlib_decompress returned an unknown error type: this is an internal bug that **must** be fixed");
                }
            }
        }
    }
}

impl Drop for Decompressor {
    fn drop(&mut self) {
        unsafe {
            libdeflate_free_decompressor(self.p);
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct CompressionLvl(i32);

impl CompressionLvl {
    pub fn new(level: i32) -> CompressionLvl {
        CompressionLvl(level)
    }

    pub fn fastest() -> CompressionLvl {
        CompressionLvl(1)
    }

    pub fn best() -> CompressionLvl {
        CompressionLvl(12)
    }

    pub fn iter() -> CompressionLvlIter {
        CompressionLvlIter(1)
    }
}

impl Default for CompressionLvl {
    fn default() -> CompressionLvl {
        CompressionLvl(6)
    }
}

pub struct CompressionLvlIter(i32);

impl Iterator for CompressionLvlIter {
    type Item = CompressionLvl;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= 12 {
            let ret = Some(CompressionLvl(self.0));
            self.0 += 1;
            ret
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CompressionError {
    InsufficientSpace,
}

type CompressionResult<T> = std::result::Result<T, CompressionError>;

pub struct Compressor {
    p: *mut libdeflate_compressor,
}

impl Compressor {
    pub fn new(lvl: CompressionLvl) -> Compressor {
        unsafe {
            let ptr = libdeflate_alloc_compressor(lvl.0);
            if !ptr.is_null() {
                Compressor{ p: ptr }
            } else {
                panic!("libdeflate_alloc_decompressor returned NULL: out of memory");
            }
        }
    }

    pub fn compress_deflate_bound(&mut self, n_bytes: usize) -> usize {
        unsafe {
            libdeflate_deflate_compress_bound(self.p, n_bytes)
        }
    }

    pub fn compress_deflate(&mut self,
                            in_raw_data: &[u8],
                            out_deflate_data: &mut [u8]) -> CompressionResult<usize> {
        unsafe {
            let in_ptr = in_raw_data.as_ptr() as *const std::ffi::c_void;
            let out_ptr = out_deflate_data.as_mut_ptr() as *mut std::ffi::c_void;

            let sz = libdeflate_deflate_compress(self.p,
                                                 in_ptr,
                                                 in_raw_data.len(),
                                                 out_ptr,
                                                 out_deflate_data.len());

            if sz != 0 {
                Ok(sz)
            } else {
                Err(CompressionError::InsufficientSpace)
            }
        }
    }

    pub fn compress_zlib_bound(&mut self, n_bytes: usize) -> usize {
        unsafe {
            libdeflate_zlib_compress_bound(self.p, n_bytes)
        }
    }

    pub fn compress_zlib(&mut self,
                         in_raw_data: &[u8],
                         out_zlib_data: &mut [u8]) -> CompressionResult<usize> {
        unsafe {
            let in_ptr = in_raw_data.as_ptr() as *const std::ffi::c_void;
            let out_ptr = out_zlib_data.as_mut_ptr() as *mut std::ffi::c_void;

            let sz = libdeflate_zlib_compress(self.p,
                                              in_ptr,
                                              in_raw_data.len(),
                                              out_ptr,
                                              out_zlib_data.len());

            if sz != 0 {
                Ok(sz)
            } else {
                Err(CompressionError::InsufficientSpace)
            }
        }
    }

    pub fn compress_gzip_bound(&mut self, n_bytes: usize) -> usize {
        unsafe {
            libdeflate_gzip_compress_bound(self.p, n_bytes)
        }
    }

    pub fn compress_gzip(&mut self,
                         in_raw_data: &[u8],
                         out_gzip_data: &mut [u8]) -> CompressionResult<usize> {
        unsafe {
            let in_ptr = in_raw_data.as_ptr() as *const std::ffi::c_void;
            let out_ptr = out_gzip_data.as_mut_ptr() as *mut std::ffi::c_void;

            let sz = libdeflate_gzip_compress(self.p,
                                              in_ptr,
                                              in_raw_data.len(),
                                              out_ptr,
                                              out_gzip_data.len());

            if sz != 0 {
                Ok(sz)
            } else {
                Err(CompressionError::InsufficientSpace)
            }
        }
    }
}

impl Drop for Compressor {
    fn drop(&mut self) {
        unsafe {
            libdeflate_free_compressor(self.p);
        }
    }
}
