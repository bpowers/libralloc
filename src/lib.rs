#![feature(lang_items)]
#![feature(libc)]
#![feature(link_args)]

#![feature(alloc_jemalloc)]

#![warn(unused_variables)]

extern crate alloc_jemalloc;

extern crate libc;

use std::sync::{Once, ONCE_INIT};
use libc::{size_t, c_void};

// aligned_alloc
// cfree
// memalign
// posix_memalign
// prealloc ??

//static

static INIT: Once = ONCE_INIT;
//INIT.call_once(|| {
//
//});

#[no_mangle]
pub extern fn malloc_usable_size(ptr: *mut c_void) -> size_t {
    println!("malloc_usable_size(_)");
    return 0;
}

#[no_mangle]
pub extern fn malloc(size: size_t) -> *mut c_void {
    println!("malloc(_)");
    return 0 as *mut libc::c_void;
}

#[no_mangle]
pub extern fn calloc(nmemb: size_t, size: size_t) -> *mut c_void {
    //println!("calloc(_, _)");
    return 0 as *mut libc::c_void;
}

#[no_mangle]
pub extern fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    println!("realloc(_, _)");
    return ptr;
}

#[no_mangle]
pub extern fn free(ptr: *mut c_void) {
    println!("free(_)");
}
