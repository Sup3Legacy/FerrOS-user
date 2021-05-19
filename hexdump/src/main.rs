#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::{io, syscall};

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
    let mut name = None;
    let mut pos = 1;
    let mut length = usize::MAX;
    let mut canonical = false;
    while pos < args.len() - 1 {
        if args[pos].len() == 0 {
            pos += 1;
        } else if args[pos].as_bytes()[0] == b'-' {
            if args[pos] == "-n" {
                match str::parse(&args[pos+1]) {
                    Ok(n) => length = n,
                    Err(_) => {
                        io::_print(&String::from("-n awaits a integer\n"));
                        unsafe {
                            syscall::exit(3);
                        }
                    },
                }
                pos += 2;
            } else if args[pos] == "-C" {
                pos += 1;
                canonical = true;
            } else {
                io::_print(&String::from("Unknown flag\n"));
                unsafe {
                    syscall::exit(3);
                }
            }
        } else {
            if name.is_none() {
                name = Some(&args[pos]);
            } else {
                io::_print(&String::from("Only one file can be given\n"));
                unsafe {
                    syscall::exit(3);
                }
            }
            pos += 1
        }
    }
    match name {
        None => {
            io::_print(&String::from("Needs at least one argument\n"));
            unsafe {
                syscall::exit(3);
            }
        },
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

            let file = unsafe { read_all(&s, length) };
            print_dump(&file, canonical);
        }
    }
}

unsafe fn read_all(path: &String, length: usize) -> Vec<u8> {
    let mut res = Vec::new();
    let fd = ferr_os_librust::syscall::open(&path.clone(), io::OpenFlags::ORD);
    loop {
        //ferr_os_librust::io::_print(&String::from("Reading...."));
        let partial = ferr_os_librust::io::read_input(fd, core::cmp::min(512, length - res.len()));
        let len = partial.len();
        for i in 0..core::cmp::min(len, length - res.len()) {
            res.push(partial[i]);
        }
        
        if len == 0 {
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
