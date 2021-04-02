#![no_std]
#![feature(start)]
#![feature(asm)]
#![no_main]

use core::panic::PanicInfo;
use x86_64::VirtAddr;


#[no_mangle]
pub extern "C" fn _start() {
    syscall(0, 0, 0);
    main();
}

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    unsafe { asm!("push rax", "mov rax, 9", "mov rdi, 1", "int 80h", "pop rax") }
    unsafe { asm!("push 0", "ret") }
    loop {}
}

#[inline(never)]
fn main() {
    syscall(0, 0, 0);
    print("hello world");
    syscall(2, 0, 0);
    panic!("failure");
}

fn print(a: &str) {
    let mut t : [u8; 128] = [0; 128];
    //syscall(20, 42, 0);
    let mut index = 0_usize;
    
    for c in a.bytes() {
        syscall(20, index as u64, c as u64);
        t[index] = c;
        index += 1;
        if index == 128 {
            t[index - 1] = 0; // We put a guard
            break
        }
    }
    syscall(1, 0, 0);
    syscall_write(1, VirtAddr::from_ptr(t.as_ptr()).as_u64(), index as u64);
}

extern "C" fn syscall_write(arg0 : u64, arg1 : u64, arg2 : u64) -> usize{
    let res;
    unsafe {
        asm!(
            "push rax",
            "push rdi",
            "push rsi",
            "mov rax, 1",
            "mov rdi, {}", 
            "mov rsi, {}",
            "mov rdx, {}",
            "int 80h",
            "pop rsi",
            "pop rdi",
            "mov {}, rax",
            "pop rax", 
            in(reg) arg0, in(reg) arg1, in(reg) arg2, out(reg) res)
    };
    res
}

extern "C" fn syscall(nb: u64, arg0 : u64, arg1 : u64) -> usize{
    let res;
    unsafe {
        asm!(
            "push rax",
            "push rdi",
            "push rsi",
            "mov rax, {}", 
            "mov rdi, {}",
            "mov rsi, {}",
            "int 80h",
            "pop rsi",
            "pop rdi",
            "mov {}, rax", 
            "pop rax", 
            in(reg) nb, in(reg) arg0, in(reg) arg1, out(reg) res)
    };
    res
}