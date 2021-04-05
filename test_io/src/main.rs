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
use x86_64::VirtAddr;
mod allocator;
mod serial;

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;


#[no_mangle]
pub extern "C" fn _start() {
    syscall(20, 51, 0, 0);
    //println!("Whelp!");
    main();
}

#[inline(never)]
fn main() {
    let mut buffer = [0_u8; 256];
    let mut a = String::new();
    loop {
        String::clear(&mut a);
        let length = syscall(0, 0, buffer.as_ptr() as u64, 256); 
        for i in 0..length {
            a.push(buffer[i] as char);
        }
        print(&a);
        halt();
    }
}


fn print(a: &String) {
    syscall(20, 12, 0, 0);
    let mut t: [u8; 128] = [0; 128];
    //syscall(20, 42, 0);
    let mut index = 0_usize;

    for c in a.bytes() {
        //syscall(20, index as u64, c as u64, 0);
        t[index] = c;
        index += 1;
        if index == 128 {
            t[index - 1] = 0; // We put a guard
            break;
        }
    }
    let data_addr = VirtAddr::from_ptr(&t as *const u8);
    syscall(1, 1, data_addr.as_u64(), index as u64);
}

pub fn halt() {
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

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    unsafe {
        syscall(20, 420, 0, 0);
        asm!("push 1", "ret");
    }
    loop {}
}