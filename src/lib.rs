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
