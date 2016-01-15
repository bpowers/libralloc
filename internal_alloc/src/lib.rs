#![feature(allocator)]
#![allocator]

// Allocators are not allowed to depend on the standard library which
// in turn requires an allocator in order to avoid circular
// dependencies. This crate, however, can use all of libcore.
#![no_std]

#![crate_type = "rlib"]

// Our system allocator will use the in-tree libc crate for FFI
// bindings. Note that currently the external (crates.io) libc cannot
// be used because it links to the standard library (e.g. `#![no_std]`
// isn't stable yet), so that's why this specifically requires the
// in-tree version.
#![feature(libc)]
extern crate libc;

extern crate slabmalloc;

use core::mem::{transmute};
use slabmalloc::{ZoneAllocator, SlabPage, SlabPageProvider};

const BASE_PAGE_SIZE: usize = 4096;

/// Page allocator based on mmap/munmap system calls for backing slab memory.
struct MmapPageProvider {
    currently_allocated: usize
}

impl MmapPageProvider {
    pub fn new() -> MmapPageProvider {
        MmapPageProvider{ currently_allocated: 0 }
    }
}

impl MmapPageProvider {
    pub fn currently_allocated(&self) -> usize {
        self.currently_allocated
    }
}

impl<'a> SlabPageProvider<'a> for MmapPageProvider {

    /// Allocates a new SlabPage from the system.
    ///
    /// Uses `mmap` to map a page and casts it to a SlabPage.
    fn allocate_slabpage(&mut self) -> Option<&'a mut SlabPage<'a>> {
        let mut addr: libc::c_void = libc::c_void::__variant1;
        let len: libc::size_t = BASE_PAGE_SIZE;
        let prot = libc::PROT_READ | libc::PROT_WRITE;
        let flags = libc::MAP_PRIVATE | libc::MAP_ANON;
        let fd = -1;
        let offset = 0;
        let r = unsafe { libc::mmap(&mut addr, len as libc::size_t, prot, flags, fd, offset) };
        if r == libc::MAP_FAILED {
            return None;
        }
        else {
            let mut slab_page: &'a mut SlabPage = unsafe { transmute(r as usize) };
            self.currently_allocated += 1;
            return Some(slab_page);
        }
    }

    /// Release a SlabPage back to the system.slab_page
    ///
    /// Uses `munmap` to release the page back to the OS.
    fn release_slabpage(&mut self, p: &'a mut SlabPage<'a>) {
        let addr: *mut libc::c_void = unsafe { transmute(p) };
        let len: libc::size_t = BASE_PAGE_SIZE;
        let r = unsafe { libc::munmap(addr, len) };
        if r != 0 {
            panic!("munmap failed!");
        }
        self.currently_allocated -= 1;
    }

}

static mut MMAP:  &'static MmapPageProvider = MmapPageProvider::new();
static mut ZONES: &'static ZoneAllocator<'static> = ZoneAllocator::new(Some(MMAP));

// Listed below are the five allocation functions currently required
// by custom allocators. Their signatures and symbol names are not
// currently typechecked by the compiler, but this is a future
// extension and are required to match what is found below.
//
// Note that the standard `malloc` and `realloc` functions do not
// provide a way to communicate alignment so this implementation would
// need to be improved with respect to alignment in that aspect.

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe { libc::malloc(size as libc::size_t) as *mut u8 }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe { libc::free(ptr as *mut libc::c_void) }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                _align: usize) -> *mut u8 {
    unsafe {
        libc::realloc(ptr as *mut libc::c_void, size as libc::size_t) as *mut u8
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // this api is not supported by libc
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
