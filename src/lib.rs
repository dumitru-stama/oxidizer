#![feature(alloc_error_handler, box_syntax, lang_items)]
#![no_std]
#[macro_use]

extern crate alloc;
extern crate libc;
mod magic;
mod layer;

use magic::CAlloc;
use core::panic::PanicInfo;
use core::alloc::*;

use layer::calculate_something;

#[global_allocator]
static ALLOCATOR: CAlloc = CAlloc;

//----------------------------------------------------------------------
// One export example
//----------------------------------------------------------------------
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

