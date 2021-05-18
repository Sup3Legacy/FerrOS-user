#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::{io, syscall};

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
    let proc_fd = unsafe { syscall::open(&String::from("/proc"), io::OpenFlags::ORD) };
    let procs_str = io::read_to_string(proc_fd, 1024);
    let proc = procs_str
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    io::_print(&String::from("ID PPID  Heap-add  Heap (kiB)           State                 name\n"));
    for id in proc {
        let ppid = get_info(id, "ppid");
        let heap_base = get_info(id, "heap");
        let heap_total = heap_base
            .split_whitespace()
            .collect::<Vec<&str>>();
        let heap_address = heap_total[0];
        let heap_size = heap_total[1];
        let state = get_info(id, "state");
        let name = get_info(id, "name");
        io::_print(&format!("{:>2?} {:>4} {:>10} {:>10} {:>15} {:>20}\n", id, ppid, heap_address, heap_size, state, name));
    }
}

fn get_info(proc: u32, info: &str) -> String {
    let path = format!("/proc/{}/{}", proc, info);
    let fd = unsafe { syscall::open(&path, io::OpenFlags::ORD) };
    let res = io::read_to_string(fd, 1024);
    unsafe { syscall::close(fd) };
    res
}
