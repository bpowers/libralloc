#![feature(lang_items)]
#![feature(libc)]


extern crate libc;

extern crate internal_alloc;

// aligned_alloc
// calloc
// cfree
// free
// malloc
// malloc_usable_size
// memalign
// posix_memalign
// prealloc

#[no_mangle]
pub extern fn malloc(size: libc::size_t) -> *mut libc::c_void {
    return 0 as *mut libc::c_void;
}
