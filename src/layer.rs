use core::panic::PanicInfo;
extern crate libc;

extern {
    fn stub_function(input: libc::c_int) -> libc::c_int;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn calculate_something(v: libc::c_int) -> libc::c_int {
    let res: libc::c_int;
    unsafe {
        res = stub_function(v);
    }
    res*res
}

