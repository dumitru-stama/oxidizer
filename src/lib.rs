#![no_std]

extern crate libc;
extern crate mbox;
extern crate sha2;
extern crate basenc;

mod layer;

use layer::{calculate_something, print_under_120, get_file_contents};
use mbox::{MBox, MArray};
use sha2::{Sha256, Digest};

#[no_mangle]
pub extern fn respond_promptly(v: libc::c_int) -> libc::c_int {
    let mut hasher = Sha256::new();
    let data = b"Testing string";
    let mdata = MArray::from_slice(&data[..]);


    let fc = get_file_contents(b"testfile.bin");
    
    
    let mut buff: [u8;128] = [0;128];
    let mut hash_bytes: [u8;32] = [0;32];
    //hasher.input(mdata);
    hasher.input(fc);
    let hash = hasher.result();
    let mut i = 0;
    for c in hash {
        hash_bytes[i] = c as u8;
        i += 1;
    }
    
    basenc::encode(&hash_bytes, basenc::UpperHex, & mut buff[..]);
    buff[64] = 0;
    
    print_under_120(&buff[..], 64);

    let five = MBox::new(5);
    calculate_something(v) * calculate_something(*five)
}

