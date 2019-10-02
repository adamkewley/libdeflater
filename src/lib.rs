//! Rust bindings to [`libdeflate`], a DEFLATE-based buffer
//! compression/decompression library that works with raw DEFLATE,
//! zlib, and gzip data.
//!
//! **Warning**: Libdeflate is targeted at *specialized*
//! performance-sensitive use-cases where developers have a good
//! understanding of their input/output data. Developers looking for a
//! general-purpose DEFLATE library should use something like
//! [`flate2`], which can handle a much wider range of inputs (network
//! streams, large files, etc.).
//!
//! [`libdeflate`]: https://github.com/ebiggers/libdeflate
//! [`flate2`]: https://github.com/alexcrichton/flate2-rs
//!
//! # Decompression
//!
//! [`Decompressor::new`] can be used to construct a [`Decompressor`],
//! which can decompress:
//!
//! - DEFLATE data ([`decompress_deflate`])
//! - zlib data ([`decompress_zlib`])
//! - gzip data ([`decompress_gzip`])
//!
//! **Note**: `libdeflate` requires that the input *and* output
//! buffers are pre-allocated before decompressing. Because of this,
//! you will at least need to know the upper bound on how large the
//! compressed data will decompress to; otherwise, a `decompress_*`
//! function call will return `DecompressionError::InsufficientSpace`
//!
//! [`Decompressor::new`]: struct.Decompressor.html#method.new
//! [`Decompressor`]: struct.Decompressor.html
//! [`decompress_deflate`]: struct.Decompressor.html#method.decompress_deflate
//! [`decompress_zlib`]: struct.Decompressor.html#method.decompress_zlib
//! [`decompress_gzip`]: struct.Decompressor.html#method.decompress_gzip
//! [`DecompressionError::InsufficientSpace`]: enum.DecompressionError.html
//!
//! # Compression
//!
//! `Compressor::new` can be used to construct a [`Compressor`], which
//! can compress data into the following formats:
//!
//! - DEFLATE ([`compress_deflate`])
//! - zlib ([`compress_zlib`])
//! - gzip ([`compress_gzip`])
//!
//! Because buffers must be allocated up-front, developers need to
//! supply these functions with output buffers that are big enough to
//! fit the compressed data. The maximum size of the compressed data
//! can be found with the associated `*_bound` methods:
//!
//! - [`compress_deflate_bound`]
//! - [`compress_zlib_bound`]
//! - [`compress_gzip_bound`]
//!
//! [`Compressor::new`]: struct.Compressor.html#method.new
//! [`Compressor`]: struct.Compressor.html
//! [`compress_deflate`]: struct.Compressor.html#method.compress_deflate
//! [`compress_zlib`]: struct.Compressor.html#method.compress_zlib
//! [`compress_gzip`]: struct.Compressor.html#method.compress_gzip
//! [`compress_deflate_bound`]: struct.Compressor.html#method.compress_deflate_bound
//! [`compress_zlib_bound`]: struct.Compressor.html#method.compress_zlib_bound
//! [`compress_gzip_bound`]: struct.Compressor.html#method.compress_gzip_bound

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

/// A `libdeflate` decompressor that can inflate DEFLATE, zlib, or
/// gzip data.
pub struct Decompressor {
    p: *mut libdeflate_decompressor,
}

/// An error that may be returned by one of the
/// [`Decompressor`](struct.Decompressor.html)'s `decompress_*`
/// methods when a decompression cannot be performed.
#[derive(Debug, PartialEq)]
pub enum DecompressionError {
    /// The provided data is invalid in some way. For example, the
    /// checksum in the data revealed possible corruption, magic
    /// numbers in the data do not match expectations, etc.
    BadData,

    /// The provided output buffer is not large enough to accomodate
    /// the decompressed data.
    InsufficientSpace,
}

/// A result returned by decompression methods
type DecompressionResult<T> = std::result::Result<T, DecompressionError>;

#[allow(non_upper_case_globals)]
impl Decompressor {

    /// Returns a newly constructed instance of a `Decompressor`.
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

    /// Decompresses `gz_data` (a buffer containing
    /// [`gzip`](https://tools.ietf.org/html/rfc1952) data) and writes
    /// the decompressed data into `out`. Returns the number of
    /// decompressed bytes written into `out`, or an error (see
    /// [`DecompressionError`](enum.DecompressionError.html) for error
    /// cases).
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

    /// Decompresses `zlib_data` (a buffer containing
    /// [`zlib`](https://www.ietf.org/rfc/rfc1950.txt) data) and
    /// writes the decompressed data to `out`. Returns the number of
    /// decompressed bytes written into `out`, or an error (see
    /// [`DecompressionError`](enum.DecompressionError.html) for error
    /// cases).
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

    /// Decompresses `deflate_data` (a buffer containing
    /// [`deflate`](https://tools.ietf.org/html/rfc1951) data) and
    /// writes the decompressed data to `out`. Returns the number of
    /// decompressed bytes written into `out`, or an error (see
    /// [`DecompressionError`](enum.DecompressionError.html) for error
    /// cases).
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

/// Compression level used by a [`Compressor`](struct.Compressor.html)
/// instance.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct CompressionLvl(i32);

/// Errors that can be returned when attempting to create a
/// [`CompressionLvl`](enum.CompressionLvl.html) from a numeric value.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CompressionLvlError {
    InvalidValue,
}

/// A result that is returned when trying to create a
/// [`CompressionLvl`](enum.CompressionLvl.html) from a numeric value.
type CompressionLevelResult = Result<CompressionLvl, CompressionLvlError>;

impl CompressionLvl {
    /// Try to create a valid
    /// [`CompressionLvl`](enum.CompressionLvl.html) from a numeric
    /// value.
    ///
    /// If `level` is a valid custom compression level for libdeflate,
    /// returns a `Result::Ok(CompressionLvl)`. Otherwise, returns
    /// `Result::Error(error)`.
    ///
    /// Valid compression levels for libdeflate, at time of writing,
    /// are 1-12.
    pub fn new(level: i32) -> CompressionLevelResult {
        const MIN_COMPRESSION_LVL: i32 = 1;
        const MAX_COMPRESSION_LVL: i32 = 12;

        if MIN_COMPRESSION_LVL <= level && level <= MAX_COMPRESSION_LVL {
            Ok(CompressionLvl(level))
        } else {
            Err(CompressionLvlError::InvalidValue)
        }
    }

    /// Returns the fastest compression level. This compression level
    /// offers the highest performance but lowest compression ratio.
    pub fn fastest() -> CompressionLvl {
        CompressionLvl(1)
    }

    /// Returns the best compression level, in terms of compression
    /// ratio. This compression level offers the best compression
    /// ratio but lowest performance.
    pub fn best() -> CompressionLvl {
        CompressionLvl(12)
    }

    /// Returns an iterator that emits all compression levels
    /// supported by `libdeflate` in ascending order.
    pub fn iter() -> CompressionLvlIter {
        CompressionLvlIter(1)
    }
}

impl Default for CompressionLvl {
    /// Returns the default compression level reccomended by
    /// libdeflate.
    fn default() -> CompressionLvl {
        CompressionLvl(6)
    }
}

/// An iterator over the
/// [`CompressionLvl`](struct.CompressionLvl.html)s supported by the
/// [`Compressor`](struct.Compressor.html).
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

/// An error that may be returned when calling one of the
/// [`Compressor`](struct.Compressor.html)'s `compress_*` methods.
#[derive(Debug, PartialEq)]
pub enum CompressionError {
    InsufficientSpace,
}

type CompressionResult<T> = std::result::Result<T, CompressionError>;

/// A `libdeflate` compressor that can compress arbitrary data into
/// DEFLATE, zlib, or gzip formats.
pub struct Compressor {
    p: *mut libdeflate_compressor,
}

impl Compressor {

    /// Returns a newly constructed `Compressor` that compresses data
    /// with the supplied
    /// [`CompressionLvl`](struct.CompressionLvl.html)
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

    /// Returns the maximum number of bytes required to encode
    /// `n_bytes` as [`deflate`](https://tools.ietf.org/html/rfc1951)
    /// data. This is a hard upper-bound that assumes the worst
    /// possible compression ratio (i.e. assumes the data cannot be
    /// compressed), format overhead, etc.
    pub fn compress_deflate_bound(&mut self, n_bytes: usize) -> usize {
        unsafe {
            libdeflate_deflate_compress_bound(self.p, n_bytes)
        }
    }

    /// Compresses `in_raw_data` as
    /// [`deflate`](https://tools.ietf.org/html/rfc1951) data, writing
    /// the data into `out_deflate_data`. Returns the number of bytes
    /// written into `out_deflate_data`.
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

    /// Returns the maximum number of bytes required to encode
    /// `n_bytes` as [`zlib`](https://www.ietf.org/rfc/rfc1950.txt)
    /// data. This is a hard upper-bound that assumes the worst
    /// possible compression ratio (i.e. assumes the data cannot be
    /// compressed), format overhead, etc.
    pub fn compress_zlib_bound(&mut self, n_bytes: usize) -> usize {
        unsafe {
            libdeflate_zlib_compress_bound(self.p, n_bytes)
        }
    }

    /// Compresses `in_raw_data` as
    /// [`zlib`](https://www.ietf.org/rfc/rfc1950.txt) data, writing
    /// the data into `out_deflate_data`. Returns the number of bytes
    /// written into `out_deflate_data`.
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

    /// Returns the maximum number of bytes required to encode
    /// `n_bytes` as [`gzip`](https://tools.ietf.org/html/rfc1952)
    /// data. This is a hard upper-bound that assumes the worst
    /// possible compression ratio (i.e. assumes the data cannot be
    /// compressed), format overhead, etc.
    pub fn compress_gzip_bound(&mut self, n_bytes: usize) -> usize {
        unsafe {
            libdeflate_gzip_compress_bound(self.p, n_bytes)
        }
    }

    /// Compresses `in_raw_data` as
    /// [`gzip`](https://tools.ietf.org/html/rfc1952) data, writing
    /// the data into `out_deflate_data`. Returns the number of bytes
    /// written into `out_deflate_data`.
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
