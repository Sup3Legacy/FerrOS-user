#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::io;

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args_number: u64, _args: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    let arguments = ferr_os_librust::env::retrieve_arguments(args_number, _args);
    main(arguments);
}

#[inline(never)]
fn main(args: Vec<String>) {
    match args.get(1) {
        None => io::_print(&String::from(alloc::format!(
            "Could not get anything to echo. Got args : {:?}\n",
            args
        ))),
        Some(s) => io::_print(s),
    }
}
