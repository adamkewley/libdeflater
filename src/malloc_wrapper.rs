//! This is a module that provides `malloc` and `free` for `libdeflate`.
//! These implementations are compatible with the standard signatures
//! but use Rust allocator instead of including libc one as well.
//!
//! Rust allocator APIs requires passing size and alignment to the
//! `dealloc` function. This is different from C API, which only
//! expects a pointer in `free` and expects allocators to take care of
//! storing any necessary information elsewhere.
//!
//! In order to simulate C API, we allocate a `size_and_data_ptr`
//! of size `sizeof(usize) + size` where `size` is the requested number
//! of bytes. Then, we store `size` at the beginning of the allocated
//! chunk (within those `sizeof(usize)` bytes) and return
//! `data_ptr = size_and_data_ptr + sizeof(usize)` to the calleer:
//!
//! [`size`][...actual data]
//! -^------------------ `size_and_data_ptr`
//! ---------^---------- `data_ptr`
//!
//! Then, in `free`, the caller gives us `data_ptr`. We can subtract
//! `sizeof(usize)` back and get the original `size_and_data_ptr`.
//! At this point we can read `size` back and call the Rust `dealloc`
//! for the whole allocated chunk.

use libdeflate_sys::libdeflate_options;
use std::alloc::{alloc, dealloc, Layout};
use std::ffi::c_void;
use std::mem::{align_of, size_of};

fn layout_for(size: usize) -> Option<Layout> {
    Layout::from_size_align(size.checked_add(size_of::<usize>())?, align_of::<usize>()).ok()
}

unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    let layout = match layout_for(size) {
        Some(layout) => layout,
        None => return std::ptr::null_mut(),
    };
    let size_and_data_ptr = alloc(layout).cast::<usize>();
    if size_and_data_ptr.is_null() {
        return std::ptr::null_mut();
    }
    *size_and_data_ptr = size;
    size_and_data_ptr.add(1).cast::<c_void>()
}

unsafe extern "C" fn free(data_ptr: *mut c_void) {
    // The free function should "behave like" the standard free function.
    if data_ptr.is_null() {
        return;
    }

    let size_and_data_ptr = data_ptr.cast::<usize>().sub(1);
    let size = *size_and_data_ptr;

    let layout = match layout_for(size) {
        Some(layout) => layout,
        // If the layout was invalid, we couldn't have allocated it in the first place
        None => std::hint::unreachable_unchecked(),
    };
    dealloc(size_and_data_ptr.cast(), layout);
}

pub const OPTIONS: libdeflate_options = libdeflate_options {
    sizeof_options: size_of::<libdeflate_options>(),
    malloc_func: Some(malloc),
    free_func: Some(free),
};
