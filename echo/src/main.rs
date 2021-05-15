#![no_std]
#![feature(start)]
#![no_main]


use ferr_os_librust::io;

extern crate alloc;

use alloc::string::String;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, _args: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    main();
}

#[inline(never)]
fn main() {
    io::_print(&String::from("Hello world. Please implement me\n"))
}
