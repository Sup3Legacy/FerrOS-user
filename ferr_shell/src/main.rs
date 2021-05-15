#![no_std]
#![feature(start)]
#![feature(asm)]
#![no_main]

use ferr_os_librust;
use ferr_os_librust::interfaces::keyboard;
use ferr_os_librust::{io, syscall};

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;

static mut ENV: Option<BTreeMap<String, String>> = None;

pub mod compute;
pub mod remove_variables;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, _args_number: u64, _args: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    unsafe {
        let fd = syscall::open(String::from("screen/screenfull"), 0);
        syscall::dup2(io::STD_OUT, fd);
        syscall::set_screen_size(24, 80);
        syscall::set_screen_pos(1, 0);
        let mut env1 = BTreeMap::new();
        env1.insert(String::from("SHELL"), String::from("FerrSH"));
        env1.insert(String::from("PWD"), String::from("User/"));
        env1.insert(String::from("PRINT"), String::from("$(SHELL):$(PWD) >> "));
        env1.insert(String::from("PATH"), String::from("User/root/bin/"));
        ENV = Some(env1);
    }
    main();
}

#[inline(never)]
fn main() {
    loop {
        if let Some(env) = unsafe { &mut ENV } {
            let v1 = match env.get("PRINT") {
                None => "FerrSH >> ",
                Some(v) => v,
            };
            let intro = match remove_variables::main(&String::from(v1), env) {
                Err(()) => String::from("FerrSH >> "),
                Ok(unfolded) => unfolded,
            };
            let raw = get_input(&intro);
            let mut printing = String::from("\r");
            printing.push_str(&intro);
            printing.push_str(&raw);
            printing.push('\n');
            io::_print(&printing);
            match remove_variables::main(&raw, env) {
                Ok(unfolded) => compute::bash(unfolded, env),
                Err(()) => (),
            }
        } else {
            panic!("ENV not defined");
        }
    }
}

fn get_input(intro: &String) -> String {
    let mut begin = String::new();
    let mut end = String::new();

    loop {
        let v = io::read_input(io::STD_IN, 512);
        let previous_size = begin.len() + end.len();
        keyboard::translate(v, &mut begin, &mut end);
        for i in previous_size..(1 + begin.len() + end.len()) {
            io::_print(&String::from(" "));
        }
        for i in 0..begin.len() {
            if begin.as_bytes()[i] == b'\n' {
                begin.truncate(i);
                return begin;
            }
        }
        for i in 0..end.len() {
            if end.as_bytes()[i] == b'\n' {
                end.truncate(i);
                return begin + &end;
            }
        }
        io::_print(&String::from("\r"));
        io::_print(intro);
        io::_print(&begin);
        io::_print(&String::from("|"));
        io::_print(&end);
    }
}
