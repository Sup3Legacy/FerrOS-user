#![no_std]
#![feature(start)]
#![feature(asm)]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {
    syscall(0, 0, 0);
    main();
}

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    unsafe { asm!("push rax", "mov rax, 1", "int 80h", "pop rax") }
    unsafe { asm!("push 0", "ret") }
    loop {}
}

#[inline(never)]
#[allow(clippy::empty_loop)]
fn main() {
    syscall(0, 0, 0);
    syscall(0, 0, 0);
    syscall(1, 0, 0);
    syscall(2, 0, 0);
    let fork = syscall(5, 0, 0);
    // Debugs the pid of child
    syscall(20, fork as u64, 0);
    if fork == 0 {
        syscall(6, 0, 0);
    }
    loop {}
}

extern "C" fn syscall(nb: u64, arg0 : u64, arg1 : u64) -> usize{
    let res;
    unsafe {
        asm!(
            "mov rax, {}", 
            "mov rdi, {}",
            "mov rsi, {}",
            "int 80h",
            "mov {}, rax", 
            in(reg) nb, in(reg) arg0, in(reg) arg1, out(reg) res)
    };
    res
}
