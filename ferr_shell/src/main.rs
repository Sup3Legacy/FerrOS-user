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

    let mut input = String::new();
    let mut end = String::new();

    loop {
        let v = io::read_input(io::STD_IN, 512);
        let previous_size = input.len() + end.len();
        keyboard::translate(v, &mut input, &mut end);
        io::_print(&String::from("\rFerrOS >>"));
        io::_print(&input);
        io::_print(&String::from("|"));
        io::_print(&end);
        for i in previous_size..(1+input.len() + end.len()){
            io::_print(&String::from(" "));
        }
        //print!("{}", s);
    }
}
