#![feature(alloc_error_handler, box_syntax, lang_items)]
#![no_std]
#[macro_use]

extern crate alloc;
extern crate libc;
mod layer;

use core::panic::PanicInfo;
use core::alloc::*;
use layer::calculate_something;

//----------------------------------------------------------------------
// One export example
#[no_mangle]
pub extern fn functie(argc: i32, argv: *const *const u8) -> i32 {
    let mut b = box [1i32;100000];
    let v = vec![0u8;100000];
    let mut res: i32 = 0;
    let mut i = 0;
    for a in v.iter() {
        res += *a as i32 + b[i];
        b[i] += 1;
        i += 1;
    }
    calculate_something(res+b[1])
}

//----------------------------------------------------------------------
//----------------------------------------------------------------------
// Here is the magic stuff
// ---------------------------------------------------------------------
//----------------------------------------------------------------------

extern "C" {
    fn malloc(s: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

struct CAlloc;
unsafe impl GlobalAlloc for CAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        free(ptr);
    }
}

#[global_allocator]
static ALLOCATOR: CAlloc = CAlloc;

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

