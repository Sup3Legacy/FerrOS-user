#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::syscall;
use ferr_os_librust::io;


extern crate alloc;
use alloc::string::ToString;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, _args: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    unsafe {
        syscall::set_screen_size(1, 10);
        syscall::set_screen_pos(0, 0);
    }
    main();
}

#[inline(never)]
fn main() {
    let mut compteur = 0_usize;
    loop {
        let new_id = unsafe { syscall::fork() };
        compteur += 1;
        if new_id == 0 {
            return
        }
        let mut sortie = compteur.to_string();
        sortie.push_str("\n");
        io::print(&sortie);
        wait_end();
    }
}

fn wait_end() {
    loop {
        let (rax, _rdi) = unsafe { syscall::listen() };
        if rax == 0 {
            unsafe {
                syscall::sleep()
            };
        } else {
            return;
        }
    }
}
