#![no_std]
#![feature(start)]
#![no_main]


extern crate alloc;

use ferr_os_librust::syscall;
use ferr_os_librust::io;

use alloc::string::String;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64) {
    unsafe {
        syscall::debug(1, 0);
        syscall::set_screen_size(19, 79);
        syscall::set_screen_pos(1, 1);
    }
    ferr_os_librust::allocator::init(heap_address, heap_size);

    let mut a = String::new();
    a.push('a');
    unsafe {
        if syscall::fork() == 0 {
            syscall::exec(a);
        }
    }
    main();
}

#[inline(never)]
fn main() {
    let mut read_buffer = [0_u8; 256];
    let mut buffer = [0_u8; 256];

    loop {
        let length = unsafe { syscall::read(0, read_buffer.as_mut_ptr(), 256) };
        let write_length = ferr_os_librust::interfaces::keyboard::decode_buffer(
            &read_buffer[..],
            &mut buffer[..],
            length,
        );
        io::print_buffer(&buffer[..], write_length);
        unsafe {
            syscall::sleep()
        };
    }
}
