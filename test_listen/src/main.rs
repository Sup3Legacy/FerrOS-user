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

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_panicInfo : &PanicInfo) -> !{
    panic!("")
}

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args: u64) {
    main();
}

#[inline(never)]
fn main() {
    loop {
        let new_id = fork();
        if new_id == 0 {
            return
        }
        wait_end();
    }
}

fn wait_end() {
    loop {
        let (rax, rdi) = listen();
        if rax == 0 {
            sleep();
        } else {
            return;
        }
    }
}

fn fork() -> u64 {
    syscall(5, 0, 0, 0) as u64
}

fn listen() -> (usize, usize) {
    let r1;
    let r2;
    unsafe {
        asm!(
            "int 80h",
            in("rax") 22, lateout("rax") r1, lateout("rdi") r2
        )
    };
    (r1, r2)
}

fn sleep() {
    syscall(8, 0, 0, 0);
}

#[inline(never)]
pub extern "C" fn syscall(nb: u64, arg0: u64, arg1: u64, arg2: u64) -> usize {
    let res;
    unsafe {
        asm!(
            "int 80h",
            in("rax") nb, in("rdi") arg0, in("rsi") arg1, in("rdx") arg2, lateout("rax") res)
    };
    res
}