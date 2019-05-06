extern crate libc;


extern "C" {
    fn stub_function(input: libc::c_int) -> libc::c_int;
    fn print_function(input: *const[i8;128]);
    //fn read_10_mb(filename: *mut[u8;1024]) -> *mut u8;
}

#[no_mangle]
pub extern fn calculate_something(v: libc::c_int) -> libc::c_int {
    let res: libc::c_int;
    unsafe {
        res = stub_function(v);
    }
    res*res
}

