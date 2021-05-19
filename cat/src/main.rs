#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::{io, syscall};

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::{
        String,
        ToString
        };

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args: u64, args_number: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    let arguments = ferr_os_librust::env::retrieve_arguments(args_number, args);
    main(arguments);
}

#[inline(never)]
fn main(args: Vec<String>) {
    if args.len() < 3 {
        io::_print(&String::from("Didn't get any argument \n"));
        return;
    }
    let mut name = None;
    let mut pos = 1;
    let mut length = usize::MAX;
    while pos < args.len() - 1 {
        if args[pos].is_empty() {
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
        None => io::_print(&String::from("No file to read\n")),
        Some(s_1) => {
            let s;
            if s_1.is_empty() {
                return;
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
            if let Some(c) = args.get(2) {
                print_dump(&file, &c[..] == "-C");
            } else {
                print_dump(&file, false);
            }
        }
    }
}

unsafe fn read_all(path: &str, length: usize) -> Vec<u8> {
    let mut res = Vec::new();
    let fd = ferr_os_librust::syscall::open(&path.to_string(), io::OpenFlags::ORD);
    loop {
        //ferr_os_librust::io::_print(&String::from("Reading...."));
        let partial = ferr_os_librust::io::read_input(fd, core::cmp::min(512, length - res.len()));
        let len = partial.len();
        for elt in partial.iter().take(core::cmp::min(len, length - res.len())) {
            res.push(*elt);
        }
        
        if len == 0 {
            break;
        }
    }
    ferr_os_librust::syscall::debug(40, res.len());
    res
}

fn print_dump(file: &[u8], _cannonical: bool) {
    let mut res = String::new();
    for u in file.iter() {
        let c = *u as char;
        res.push(c)
    }
    res.push('\n');
    ferr_os_librust::io::_print(&res);
}
