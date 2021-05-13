#![no_std]
#![feature(start)]
#![no_main]

extern crate alloc;

use ferr_os_librust::io;
use ferr_os_librust::syscall;

use alloc::string::String;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64) {
    unsafe {
        syscall::debug(1, 0);
        syscall::set_screen_size(20, 80);
        syscall::set_screen_pos(0, 0);
    }
    ferr_os_librust::allocator::init(heap_address, heap_size);

    main();
}

#[inline(never)]
fn main() {
    let mut read_buffer = [0_u8; 256];
    let mut buffer = [0_u8; 256];

    let fd = unsafe { ferr_os_librust::syscall::open(String::from("User/root/usr/issou.txt"), 0) };
    let file = &ferr_os_librust::io::read_to_string(fd, 512);
    io::print(&file);

    loop {
        unsafe {
            ferr_os_librust::syscall::sleep();
        }
    }
}
