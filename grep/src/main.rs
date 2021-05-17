#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::io;

extern crate alloc;

use alloc::vec::Vec;
use alloc::{fmt::format, string::String};

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args: u64, args_number: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    let arguments = ferr_os_librust::env::retrieve_arguments(args_number, args);
    main(arguments);
}

#[inline(never)]
fn main(args: Vec<String>) {
    unsafe {
        ferr_os_librust::syscall::debug(args[0].len(), args[1].len());
    }
    if args.len() < 3 {
        io::_print(&String::from("Didn't get any argument \n"));
        return
    }
    match args.get(1) {
        None => io::_print(&String::from("Needs at least one argument\n")),
        Some(s_1) => {
            let s;
            if s_1.len() == 0 {
                return
            } else if s_1.as_bytes()[0] == b'/' {
                s = String::from(s_1);
            } else if s_1.len() > 2 && s_1.as_bytes()[0] == b'.' && s_1.as_bytes()[1] == b'/' {
                let mut pwd = String::from(&args[args.len() - 1]);
                pwd.push_str(&s_1[2..]);
                s = pwd;
            } else {
                let mut pwd = String::from(&args[args.len() - 1]);
                pwd.push_str(s_1);
                s = pwd;
            };

            let file = unsafe { read_all(&s) };
            if let Some(c) = args.get(2) {
                print_dump(&file, &c[..] == "-C");
            } else {
                print_dump(&file, false);
            }
        }
    }
}

unsafe fn read_all(path: &String) -> Vec<u8> {
    let mut res = Vec::new();
    let fd = ferr_os_librust::syscall::open(path.clone(), 0);
    loop {
        //ferr_os_librust::io::_print(&String::from("Reading...."));
        let partial = ferr_os_librust::io::read_input(fd, 128);
        let len = partial.len();
        ferr_os_librust::syscall::debug(12, len);
        for e in partial {
            res.push(e);
        }
        if len < 128 {
            break;
        }
    }
    ferr_os_librust::syscall::debug(40, res.len());
    res
}

fn print_dump(file: &Vec<u8>, cannonical: bool) {
    let len = file.len();
    let mut address = 0_usize;
    loop {
        let mut partial = String::new();
        let address_str = alloc::format!("{:08x}  ", address);
        partial.push_str(&address_str);
        for i in 0..16 {
            if address + i >= len {
                for _ in 0..(16 - i) {
                    partial.push_str("   ");
                }
                break;
            }
            partial.push_str(&alloc::format!("{:02x} ", file[address + i]));
        }
        if cannonical {
            partial.push_str("    |");
            for i in 0..16 {
                if address + i >= len {
                    for _ in 0..(16 - i) {
                        partial.push(' ');
                    }
                    break;
                }
                let character = file[address + i] as char;
                if char::is_ascii(&character) {
                    partial.push(character);
                } else {
                    partial.push('.');
                }
            }
            partial.push('|');
        }
        partial.push('\n');
        ferr_os_librust::io::_print(&partial);
        address += 16;
        if address >= len {
            break;
        }
    }
    ferr_os_librust::io::_print(&String::from("\n"));
}
