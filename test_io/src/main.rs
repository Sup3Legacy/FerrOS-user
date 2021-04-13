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
use core::panic::PanicInfo;
use x86_64::VirtAddr;
mod serial;

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args: u64) {
    syscall(20, heap_address, heap_size, 0);
    ferr_os_librust::allocator::init(heap_address, heap_size);
    let mut a = String::new();
    a.push('a');
    //print(&a);
    //println!("Whelp!");
    main();
}

#[inline(never)]
fn main() {
    let read_buffer = [0_u8; 256];
    let mut buffer = [0_u8; 256];
    let clock_fd = open(String::from("hardware/clock"));
    let sound_fd = open(String::from("hardware/sound"));
    //push_sound(sound_fd as u64, 440, 64, 24);
    let frequencies: [u64; 13] = [
        262, 277, 293, 311, 329, 349, 369, 391, 415, 440, 466, 494, 523,
    ];
    for (i, freq) in frequencies.iter().enumerate() {
        push_sound(sound_fd as u64, *freq, 8, 24 + (i as u64) * 12);
    }

    push_sound(sound_fd as u64, 440, 40, 5 + 14 * 12);
    push_sound(sound_fd as u64, 880, 20, 5 + 14 * 12);

    loop {
        print(&read_to_string(clock_fd, 256));
        print(&String::from("\n"));
        syscall(8, 0, 0, 0);
    }
}

fn open(path: String) -> usize {
    let bytes = path.as_bytes();
    let mut buffer = [0_u8; 512];
    let length = bytes.len();
    // TODO add guards
    for i in 0..length {
        buffer[i] = bytes[i]
    }
    buffer[length] = 0_u8;
    let addr = VirtAddr::from_ptr(&buffer as *const u8);
    syscall(2, addr.as_u64(), 0, 0)
}

fn push_sound(fd: u64, tone: u64, length: u64, begin: u64) {
    let sound_buffer: [u8; 24] = unsafe { mem::transmute([tone, length, begin]) };
    let addr = VirtAddr::from_ptr(&sound_buffer as *const u8);
    syscall(1, fd, addr.as_u64(), 24);
}

fn read_to_string(fd: usize, length: usize) -> String {
    let buffer = [0_u8; 512];
    let addr = VirtAddr::from_ptr(&buffer as *const u8);
    let got = syscall(0, fd as u64, addr.as_u64(), length as u64);
    let mut res = String::new();
    for i in 0..got {
        if buffer[i] == 0 {
            break;
        }
        res.push(buffer[i] as char);
    }
    res
}

fn print_buffer(buffer: &[u8], size: usize) {
    let mut index = 0_usize;
    let mut t: [u8; 256] = [0; 256];

    for c in 0..size {
        //syscall(20, index as u64, c as u64, 0);
        t[c] = buffer[c];
    }
    let data_addr = VirtAddr::from_ptr(&t as *const u8);
    syscall(1, 1, data_addr.as_u64(), size as u64);
}

fn print(a: &String) {
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
