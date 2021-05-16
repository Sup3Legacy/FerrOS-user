#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::io;

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

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
    match args.get(1) {
        None => io::_print(&String::from("Needs at least one argument\n")),
        Some(s) => {
            let file = unsafe { read_all(s) };
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
        let partial = ferr_os_librust::io::read_input(fd, 512);
        if partial.len() == 0 {
            break;
        }
        for e in partial {
            res.push(e);
        }
    }
    res
}

fn print_dump(file: &Vec<u8>, cannonical: bool) {
    let mut res = String::new();
    let len = file.len();
    let mut address = 0_usize;
    loop {
        let mut partial = String::new();
        partial.push_str(&alloc::format!("{:#08x}", address));
        for i in 0..16 {
            if address + i >= len {
                for _ in 0..(16 - i) {
                    partial.push_str("   ");
                }
                break;
            }
            partial.push_str(&alloc::format!("{:#02x}", file[address + i]));
        }
        if cannonical {
            partial.push_str("    |");
            for i in 0..16 {
                if address + i >= len {
                    for _ in 0..(16 - i) {
                        partial.push_str(" ");
                    }
                    break;
                }
                let character = file[address + i] as char;
                if char::is_ascii_graphic(&character) {
                    partial.push(character);
                } else {
                    partial.push('.');
                }
            }
            partial.push('|');
        }
        partial.push('\n');
        res.push_str(&partial);
        address += 10;
        if address >= len {
            break;
        }
    }
    ferr_os_librust::io::_print(&res);
}
