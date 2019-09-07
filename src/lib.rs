mod libdeflate_sys;

#[allow(non_upper_case_globals)]
pub mod inflate {
    use crate::libdeflate_sys::{libdeflate_decompressor,
                                libdeflate_alloc_decompressor,
                                libdeflate_free_decompressor,
                                libdeflate_gzip_decompress,
                                libdeflate_zlib_decompress,
                                libdeflate_deflate_decompress,
                                libdeflate_result,
                                libdeflate_result_LIBDEFLATE_SUCCESS,
                                libdeflate_result_LIBDEFLATE_BAD_DATA,
                                libdeflate_result_LIBDEFLATE_INSUFFICIENT_SPACE};

    pub struct Decompressor {
        p: *mut libdeflate_decompressor,
    }

    #[derive(Debug, PartialEq)]
    pub enum Error {
        BadData,
        InsufficientSpace,
    }

    type Result<T> = std::result::Result<T, Error>;

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

        pub fn decompress_gz(&mut self,
                             gz_data: &[u8],
                             out: &mut [u8]) -> Result<usize> {
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
                        Err(Error::BadData)
                    },
                    libdeflate_result_LIBDEFLATE_INSUFFICIENT_SPACE => {
                        Err(Error::InsufficientSpace)
                    },
                    _ => {
                        panic!("libdeflate_gzip_decompress returned an unknown error type: this is an internal bug that **must** be fixed");
                    }
                }
            }
        }

        pub fn decompress_zlib(&mut self,
                               zlib_data: &[u8],
                               out: &mut [u8]) -> Result<usize> {
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
                        Err(Error::BadData)
                    },
                    libdeflate_result_LIBDEFLATE_INSUFFICIENT_SPACE => {
                        Err(Error::InsufficientSpace)
                    },
                    _ => {
                        panic!("libdeflate_zlib_decompress returned an unknown error type: this is an internal bug that **must** be fixed");
                    }
                }
            }
        }

        pub fn decompress_deflate(&mut self,
                                  deflate_data: &[u8],
                                  out: &mut [u8]) -> Result<usize> {
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
                        Err(Error::BadData)
                    },
                    libdeflate_result_LIBDEFLATE_INSUFFICIENT_SPACE => {
                        Err(Error::InsufficientSpace)
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
}

#[allow(non_upper_case_globals)]
pub mod deflate {
    use std::slice::Iter;
    use crate::libdeflate_sys::{libdeflate_compressor,
                                libdeflate_alloc_compressor,
                                libdeflate_deflate_compress_bound,
                                libdeflate_deflate_compress,
                                libdeflate_zlib_compress_bound,
                                libdeflate_free_compressor};

    #[derive(Copy, Clone)]
    pub enum CompressionLvl {
        Lvl1 = 1,
        Lvl2,
        Lvl3,
        Lvl4,
        Lvl5,
        Lvl6,
        Lvl7,
        Lvl8,
        Lvl9,
        Lvl10,
        Lvl11,
        Lvl12,
    }

    impl CompressionLvl {
        pub fn iterator() -> Iter<'static, CompressionLvl> {
            static compression_lvls: [CompressionLvl; 12] = [
                CompressionLvl::Lvl1,
                CompressionLvl::Lvl2,
                CompressionLvl::Lvl3,
                CompressionLvl::Lvl4,
                CompressionLvl::Lvl5,
                CompressionLvl::Lvl6,
                CompressionLvl::Lvl7,
                CompressionLvl::Lvl8,
                CompressionLvl::Lvl9,
                CompressionLvl::Lvl10,
                CompressionLvl::Lvl11,
                CompressionLvl::Lvl12
            ];

            compression_lvls.into_iter()
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Error {
        InsufficientSpace,
    }

    type Result<T> = std::result::Result<T, Error>;

    pub struct Compressor {
        p: *mut libdeflate_compressor,
    }

    impl Compressor {
        pub fn with_fastest_compression_lvl() -> Compressor {
            Compressor::with_compression_lvl(CompressionLvl::Lvl1)
        }
        
        pub fn with_default_compression_lvl() -> Compressor {
            Compressor::with_compression_lvl(CompressionLvl::Lvl6)
        }

        pub fn with_slow_compression_lvl() -> Compressor {
            Compressor::with_compression_lvl(CompressionLvl::Lvl9)
        }

        pub fn with_slowest_compression_lvl() -> Compressor {
            Compressor::with_compression_lvl(CompressionLvl::Lvl12)
        }
        
        pub fn with_compression_lvl(lvl: CompressionLvl) -> Compressor {
            unsafe {
                let ptr = libdeflate_alloc_compressor(lvl as i32);
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
                                out_deflate_data: &mut [u8]) -> Result<usize> {
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
                    Err(Error::InsufficientSpace)
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
                             out_zlib_data: &mut [u8]) -> Result<usize> {
            unsafe {
                let in_ptr = in_raw_data.as_ptr() as *const std::ffi::c_void;
                let out_ptr = out_zlib_data.as_mut_ptr() as *mut std::ffi::c_void;

                let sz = libdeflate_deflate_compress(self.p,
                                                     in_ptr,
                                                     in_raw_data.len(),
                                                     out_ptr,
                                                     out_zlib_data.len());

                if sz != 0 {
                    Ok(sz)
                } else {
                    Err(Error::InsufficientSpace)
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
}
