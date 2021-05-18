#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::syscall;

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, _args: u64) {
    unsafe {
        ferr_os_librust::allocator::init(heap_address, heap_size);
        let id = syscall::fork();
        if id == 0 {
            syscall::exec(&String::from("/usr/clock"), &Vec::from([String::from("clock")]));
        }
        syscall::exec(&String::from("/usr/ferr_shell"), &Vec::from([String::from("ferr_shell")]));
    }
}