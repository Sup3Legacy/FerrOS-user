#![no_std]
#![feature(start)]
#![feature(asm)]
#![no_main]


use ferr_os_librust;
use ferr_os_librust::{syscall, io, print};
use ferr_os_librust::interfaces::keyboard;

use alloc::string::String;

extern crate alloc;

static mut FIRST_WORD: bool = true;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, _args: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    unsafe {
        let fd = syscall::open(String::from("screen/screenfull"), 0);
        syscall::dup2(io::STD_OUT, fd);
        syscall::set_screen_size(24, 80);
        syscall::set_screen_pos(1, 0);
    }
    main();
}

#[inline(never)]
fn main() {

    let mut s = String::new();

    loop {
        let v = io::read_input(io::STD_IN, 512);
        keyboard::translate(v, &mut s);
        io::_print(&String::from("\r"));
        io::_print(&s);
        io::_print(&String::from(" "));
        //print!("{}", s);
    }
}
