#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(bad_style)]
mod bindings;

pub use bindings::*;

// For backwards-compatibility, we override the auto generated bindings for
// `libdeflate_set_memory_allocator` to take the malloc and free functions as function pointers
// directly rather than as `Option`s
#[inline]
pub unsafe extern "C" fn libdeflate_set_memory_allocator(
    malloc_func: unsafe extern "C" fn(arg1: usize) -> *mut ::core::ffi::c_void,
    free_func: unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void),
) {
    bindings::libdeflate_set_memory_allocator(Some(malloc_func), Some(free_func));
}

// Basic tests for Rust-to-C bindings. These tests are just for quick
// internal checks to make sure that the bindgen build script built
// something sane-looking. User-facing tests are in `tests/`
#[cfg(test)]
mod tests {
    use super::*;

    const MIN_COMP_LVL: i32 = 0;
    const MAX_COMP_LVL: i32 = 12;

    #[test]
    fn can_make_decompressor_at_each_compression_lvl() {
        unsafe {
            for lvl in MIN_COMP_LVL..MAX_COMP_LVL + 1 {
                let ptr = libdeflate_alloc_compressor(lvl);

                assert!(!ptr.is_null());

                libdeflate_free_compressor(ptr);
            }
        }
    }

    #[test]
    fn making_compressor_with_negative_compression_lvl_fails() {
        unsafe {
            let ptr = libdeflate_alloc_compressor(-1);
            assert!(ptr.is_null());
        }
    }

    #[test]
    fn making_compressor_with_too_large_compression_lvl_fails() {
        unsafe {
            let ptr = libdeflate_alloc_compressor(MAX_COMP_LVL + 1);

            assert!(ptr.is_null());
        }
    }

    #[test]
    fn can_use_compressor_to_compress_trivially_compressable_data() {
        unsafe {
            let in_data: [u8; 1 << 16] = [0; 1 << 16];
            let mut out_data: [u8; 1 << 16] = [0; 1 << 16];
            let compressor = libdeflate_alloc_compressor(MAX_COMP_LVL);
            let sz = libdeflate_deflate_compress(
                compressor,
                in_data.as_ptr() as *const core::ffi::c_void,
                in_data.len(),
                out_data.as_mut_ptr() as *mut core::ffi::c_void,
                out_data.len(),
            );
            assert_ne!(sz, 0);
            assert!(sz < 100);
        }
    }

    #[test]
    fn can_call_crc32() {
        let init = 0;
        let buf: [u8; 4] = [0x31, 0x33, 0x33, 0x37];

        unsafe {
            let ret = libdeflate_crc32(init, buf.as_ptr() as *const core::ffi::c_void, buf.len());

            assert_ne!(ret, init);
        }
    }

    #[test]
    fn can_call_adler32() {
        let init = 0;
        let buf: [u8; 4] = [0x31, 0x33, 0x33, 0x37];

        unsafe {
            let ret = libdeflate_adler32(init, buf.as_ptr() as *const core::ffi::c_void, buf.len());

            assert_ne!(ret, init);
        }
    }
}
