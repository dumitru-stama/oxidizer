use core::panic::PanicInfo;
extern crate libc;
extern crate mbox;

use mbox::{MBox, MArray};

extern {
    fn stub_function(input: libc::c_int) -> libc::c_int;
    fn print_function(input: *const[i8;128]);
    fn read_10_mb(filename: *mut[u8;1024]) -> *mut u8;
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

#[no_mangle]
pub extern fn get_file_contents(s: &[u8]) -> MBox<[u8]> {
    let mut finame: [u8;1024] = [0;1024];
    for i in 0..(s.len()) {
        if s[i] != 0 {
            finame[i] = s[i];
        } else {
            break;
        }
    }
    //MArray::from_slice(&s[..])
    let temparr;
    let marr;
    unsafe {
        temparr = read_10_mb(&mut finame);
        marr = MBox::from_raw_parts(temparr, 10000000)
    }
    //MArray::from_slice(temparr[..])
    marr
}

