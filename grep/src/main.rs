#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::io;

extern crate alloc;

use alloc::vec::Vec;
use alloc::{format, string::String};

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args: u64, args_number: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    let arguments = ferr_os_librust::env::retrieve_arguments(args_number, args);
    main(arguments);
}

#[inline(never)]
fn main(args: Vec<String>) {
    match args.get(1) {
        None => print_loop(),
        Some(s_1) => unsafe { main_loop(s_1) },
    }
}

fn print_loop() {
    loop {
        io::_print(&io::read_to_string(io::STD_IN, 512));
    }
}

unsafe fn main_loop(pattern: &String) {
    let mut line = 0;
    let mut remaining = String::new();
    loop {
        let mut inc = String::new();
        inc.push_str(&remaining);
        inc.push_str(&io::read_to_string(io::STD_IN, 512));
        remaining.clear();
        let mut splitted = inc.split('\n').collect::<Vec<&str>>();
        if splitted.len() > 0 {
            remaining.push_str(&splitted.pop().expect("Could not chop off truc"));
        }
        for s in splitted.iter() {
            if s.contains(pattern) {
                let mut res = format!("{}: ", line);
                res.push_str(s);
                res.push('\n');
                io::_print(&res);
            }
            io::_print(&String::from("Lourd"));
            line += 1;
        }
    }
}