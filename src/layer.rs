use core::panic::PanicInfo;
extern crate libc;

extern {
    fn stub_function(input: libc::c_int) -> libc::c_int;
    fn print_function(input: *const[i8;128]);
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

#[no_mangle]
pub extern fn print_under_120(s: &[u8], cnt: usize) {
    if cnt < 120 {
        let mut lc: [libc::c_char;128] = [0;128];
        for i in 0..cnt {
            lc[i] = s[i] as i8;
        }
        lc[cnt] = 0x0A as i8;
        unsafe {
            print_function(&lc);
        }
    }
}


