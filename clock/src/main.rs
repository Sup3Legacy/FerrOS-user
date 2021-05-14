#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::syscall;
use ferr_os_librust::io;

extern crate alloc;

use alloc::string::String;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, _args: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    unsafe {
        let fd = syscall::open(String::from("screen/screenfull"), 0);
        syscall::dup2(1, fd);
        syscall::set_screen_size(1, 40);
        syscall::set_screen_pos(0, 20);
    }
    main();
}

#[inline(never)]
fn main() {
    let clock_fd = unsafe { syscall::open(String::from("hardware/clock"), 0) };
    loop {
        let mut base = String::from("\n");
        base.push_str(&io::read_to_string(clock_fd, 256));
        io::_print(&base);
        unsafe {
            syscall::sleep()
        };
    }
}