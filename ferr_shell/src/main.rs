#![no_std]
#![feature(start)]
#![feature(asm)]
#![no_main]

use core::panic::PanicInfo;
use ferr_os_librust;
use x86_64::VirtAddr;

extern crate alloc;

use alloc::string::String;

#[no_mangle]
pub extern "C" fn _start() {
    main();
}

#[inline(never)]
fn main() {
    // syscall(1, 2, 0, 5);
    // let test = "command arg1 arg2".split_whitespace();
    // for s in test {
    //     print(String::from(s))
    // }
    // syscall(0, 0, 0, 0);
    print("hello");
    // print();
    // debug!("Just printed `hello`");
    // syscall(2, 0, 0, 0);
    // panic!("truc 424242");
}

fn print(a: &str) {
    syscall(1, 2, 0, 0);
    let mut t: [u8; 128] = [0; 128];
    let mut index = 0_usize;

    for c in a.bytes() {
        syscall(1, 2, 0, 0);
    }

    for c in a.bytes() {
        t[index] = c;
        index += 1;
        if index == 128 {
            t[index - 1] = 0; // We put a guard
            break;
        }
    }

    syscall(1, 2, 0, 0);

    let data_addr = VirtAddr::from_ptr(&t as *const u8);
    syscall(1, 2, data_addr.as_u64(), index as u64);
}

#[inline(never)]
extern "C" fn syscall(nb: u64, arg0: u64, arg1: u64, arg2: u64) -> usize {
    let res;
    unsafe {
        asm!(
            "mov rax, {}",
            "mov rdi, {}",
            "mov rsi, {}",
            "mov rdx, {}",
            "int 80h",
            "mov {}, rax",
            in(reg) nb, in(reg) arg0, in(reg) arg1, in(reg) arg2, out(reg) res)
    };
    res
}
