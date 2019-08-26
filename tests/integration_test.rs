use libdeflate_sys;

const MIN_COMP_LVL: i32 = 1;
const MAX_COMP_LVL: i32 = 12;

#[test]
fn can_make_decompressor_at_each_compression_lvl() {
    unsafe {
        for lvl in MIN_COMP_LVL..MAX_COMP_LVL+1 {
            let ptr = libdeflate_sys::libdeflate_alloc_compressor(lvl);
            
            assert!(!ptr.is_null());
            
            libdeflate_sys::libdeflate_free_compressor(ptr);
        }        
    }
}

#[test]
fn making_compressor_with_negative_compression_lvl_fails() {
    unsafe {
        let ptr = libdeflate_sys::libdeflate_alloc_compressor(-1);
        assert!(ptr.is_null());
    }
}

#[test]
fn making_compressor_with_too_large_compression_lvl_fails() {
    unsafe {
        let ptr = libdeflate_sys::libdeflate_alloc_compressor(MAX_COMP_LVL+1);

        assert!(ptr.is_null());
    }
}

#[test]
fn can_use_compressor_to_compress_trivially_compressable_data() {
    unsafe {
        let in_data: [u8; 1<<16] = [0; 1<<16];
        let mut out_data: [u8; 1<<16] = [0; 1<<16];
        let compressor = libdeflate_sys::libdeflate_alloc_compressor(MAX_COMP_LVL);
        let sz = libdeflate_sys::libdeflate_deflate_compress(compressor,
                                                             in_data.as_ptr() as *const core::ffi::c_void,
                                                             in_data.len(),
                                                             out_data.as_mut_ptr() as *mut core::ffi::c_void,
                                                             out_data.len());
        assert_ne!(sz, 0);
        assert!(sz < 100);
    }
}
