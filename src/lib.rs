#![no_std]
extern crate libc;
mod layer;
use layer::calculate_something;

#[no_mangle]
pub extern fn respond_promptly(v: libc::c_int) -> libc::c_int {
    calculate_something(v) * calculate_something(v+1)
}

