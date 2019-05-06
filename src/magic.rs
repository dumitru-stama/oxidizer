//#![feature(alloc_error_handler, box_syntax, lang_items)]

extern crate libc;
extern crate alloc;

use core::alloc::*;
use core::panic::PanicInfo;

extern "C" {
    fn malloc(s: usize) -> *mut u8;
    fn free(ptr: *mut u8);
    fn memset(ptr: *mut u8, c: usize, n: usize);
}

pub struct CAlloc;
unsafe impl GlobalAlloc for CAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        free(ptr);
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let buf = malloc(layout.size());
        if !buf.is_null() {
            memset(buf, 0, layout.size());
        }
        buf
    }
}

#[alloc_error_handler]
fn alloc(_: Layout) -> ! { 
    loop {} 
}

#[panic_handler]
extern "C" fn panic(_: &PanicInfo) -> ! { 
    loop {} 
}

#[no_mangle] 
pub extern fn rust_eh_register_frames () {
}

#[no_mangle] 
pub extern fn rust_eh_unregister_frames () {
}

#[lang = "eh_personality"] 
extern "C" fn eh_personality() {
}

#[lang = "eh_unwind_resume"] 
extern fn rust_eh_unwind_resume() {
}

