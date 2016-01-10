#![feature(lang_items)]
#![feature(start)]
#![feature(raw)]
#![feature(libc)]
#![no_std]


extern crate libc;

use core::mem;

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


#[lang = "panic_fmt"]
extern fn panic_fmt(args: &core::fmt::Arguments,
                    file: &str,
                    line: u32) -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
