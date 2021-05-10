#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::syscall;


#[no_mangle]
pub extern "C" fn _start(_heap_address: u64, _heap_size: u64, _args: u64) {
    main();
}

#[inline(never)]
fn main() {
    loop {
        let new_id = unsafe { syscall::fork() };
        if new_id == 0 {
            return
        }
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
