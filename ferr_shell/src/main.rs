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
    loop {
        let s = get_input();
    }
}


fn get_input() -> String {
    let mut begin = String::new();
    let mut end = String::new();

    loop {
        let v = io::read_input(io::STD_IN, 512);
        let previous_size = begin.len() + end.len();
        keyboard::translate(v, &mut begin, &mut end);
        io::_print(&String::from("\rFerrOS >>"));
        io::_print(&begin);
        io::_print(&String::from("|"));
        io::_print(&end);
        for i in previous_size..(1 + begin.len() + end.len()){
            io::_print(&String::from(" "));
        }
        for i in 0..begin.len() {
            if begin.as_bytes()[i] == b'\n' {
                begin.truncate(i);
                return begin;
            }
        }
        for i in 0..end.len() {
            if end.as_bytes()[i] == b'\n' {
                end.truncate(i);
                return begin + &end;
            }
        }
    }
}