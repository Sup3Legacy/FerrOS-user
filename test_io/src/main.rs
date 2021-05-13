#![no_std]
#![feature(start)]
#![feature(asm)]
#![no_main]
#![cfg_attr(test, no_main)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(core_intrinsics)]
#![feature(gen_future)]
#![feature(const_mut_refs)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(intra_doc_pointers)]

use core::mem;

use ferr_os_librust::syscall;
use ferr_os_librust::io;

extern crate alloc;

use alloc::string::String;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, _args: u64) {
    unsafe {
        syscall::debug(2, 0);
        syscall::set_screen_size(1, 40);
        syscall::set_screen_pos(0, 20);
    }
    ferr_os_librust::allocator::init(heap_address, heap_size);
    main();
}

#[inline(never)]
fn main() {
    let clock_fd = unsafe { syscall::open(String::from("hardware/clock"), 0) };
    unsafe {
        syscall::debug(clock_fd, 42);
    }
    /*
    let sound_fd = unsafe { syscall::open(String::from("hardware/sound"), 0) };    
    let frequencies: [u64; 13] = [
        262, 277, 293, 311, 329, 349, 369, 391, 415, 440, 466, 494, 523,
    ];
    io::push_sound(sound_fd as u64, 440, 64, 24);
    for (i, freq) in frequencies.iter().enumerate() {
        io::push_sound(sound_fd as u64, *freq, 8, 24 + (i as u64) * 12);
    }

    io::push_sound(sound_fd as u64, 440, 40, 5 + 14 * 12);
    io::push_sound(sound_fd as u64, 880, 20, 5 + 14 * 12);
    */
    loop {
        let mut base = String::from("\n");
        base.push_str(&io::read_to_string(clock_fd, 256));
        io::print(&base);
        unsafe {
            syscall::sleep()
        };
    }
}
