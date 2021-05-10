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

    /*
    let sound_fd = unsafe { syscall::open(String::from("hardware/sound"), 0) };    
    let frequencies: [u64; 13] = [
        262, 277, 293, 311, 329, 349, 369, 391, 415, 440, 466, 494, 523,
    ];
    push_sound(sound_fd as u64, 440, 64, 24);
    for (i, freq) in frequencies.iter().enumerate() {
        push_sound(sound_fd as u64, *freq, 8, 24 + (i as u64) * 12);
    }

    push_sound(sound_fd as u64, 440, 40, 5 + 14 * 12);
    push_sound(sound_fd as u64, 880, 20, 5 + 14 * 12);
    */
    loop {
        let mut base = String::from("\n");
        base.push_str(&read_to_string(clock_fd, 256));
        print(&base);
        syscall(8, 0, 0, 0);
    }
}

#[allow(dead_code)]
fn push_sound(fd: u64, tone: u64, length: u64, begin: u64) {
    let sound_buffer: [u8; 24] = unsafe { mem::transmute([tone, length, begin]) };
    unsafe {
        syscall::write(fd as usize, sound_buffer.as_ptr(), 24);
    }
}

#[allow(dead_code)]
fn read_to_string(fd: usize, length: usize) -> String {
    let mut buffer = [0_u8; 512];
    let got = unsafe { syscall::read(fd, &mut buffer as *mut u8, length) };
    let mut res = String::new();
    for i in 0..got {
        if buffer[i] == 0 {
            break;
        }
        res.push(buffer[i] as char);
    }
    res
}

#[allow(dead_code)]
fn print_buffer(buffer: &[u8], size: usize) {
    let mut t: [u8; 256] = [0; 256];

    for c in 0..size {
        //syscall(20, index as u64, c as u64, 0);
        t[c] = buffer[c];
    }
    unsafe {
        syscall::write(1, &t as *const u8, size);
    }
}

fn print(a: &String) {
    let mut t: [u8; 128] = [0; 128];
    let mut index = 0_usize;

    for c in a.bytes() {
        t[index] = c;
        index += 1;
        if index == 128 {
            t[index - 1] = 0; // We put a guard
            break;
        }
    }
    unsafe {
        syscall::write(1, &t as *const u8, index);
    }
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
