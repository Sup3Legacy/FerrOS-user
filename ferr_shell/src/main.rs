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
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args: u64, args_number: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    let arguments = ferr_os_librust::env::retrieve_arguments(args_number, args);
    unsafe {
        /*let fd = syscall::open(&String::from("/hard/screen"), io::OpenFlags::OWR);
        syscall::dup2(io::STD_OUT, fd);
        syscall::close(fd);*/
        /*syscall::set_screen_size(24, 80);
        syscall::set_screen_pos(1, 0);*/
    }

    let mut env1 = BTreeMap::new();
    env1.insert(String::from("SHELL"), String::from("FerrSH"));
    if args_number == 1 {
        env1.insert(String::from("PWD"), String::from("/"));
    } else {
        env1.insert(String::from("PWD"), String::from(&arguments[arguments.len() - 1]));
    }
    env1.insert(String::from("PS1"), String::from("$(SHELL):$(PWD) >> "));
    env1.insert(String::from("PATH"), String::from("/usr/bin/"));

    for (ind, v) in arguments.iter().enumerate() {
        let str = alloc::format!("{}", ind);
        env1.insert(str, String::from(v));
    }
    unsafe {
        ENV = Some(env1);
    }

    if args_number == 1 {
        interactive()
    } else {
        launch_file();
    }
}

fn launch_file() {
    if let Some(env) = unsafe { &mut ENV} {
        let code = read_all(io::STD_IN);
        let mut code2 = String::new();
        let mut t = false;
        for i in code.chars() {
            if t {
                code2.push(i);
            }
            if i == '\n' {
                t = true
            }
        }

        let mut line = String::new();
        for i in code2.chars() {
            if i == '\n' {
                match remove_variables::main(&line, env) {
                    Ok(unfolded) => {
                        compute::bash(unfolded, env)
                    },
                    Err(()) => {
                        unsafe {
                            syscall::exit(1);
                        }
                    }
                };
                line = String::new();
            } else {
                line.push(i)
            }
        }
        match remove_variables::main(&line, env) {
            Ok(unfolded) => {
                compute::bash(unfolded, env)
            },
            Err(()) => {
                unsafe {
                    syscall::exit(1);
                }
            }
        };
    } else {
        unsafe {
            syscall::exit(1);
        }
    }
}

#[inline(never)]
fn interactive() {
    loop {
        if let Some(env) = unsafe { &mut ENV } {
            let v1 = match env.get("PS1") {
                None => "FerrSH >> ",
                Some(v) => v,
            };
            let intro = match remove_variables::main(&String::from(v1), env) {
                Err(()) => String::from("FerrSH >> "),
                Ok(unfolded) => unfolded,
            };
            let mut raw = get_input(&intro);
            let mut printing = String::from("\r");
            printing.push_str(&intro);
            printing.push_str(&raw);
            printing.push_str(" \n");
            io::_print(&printing);
            raw.push('\n');
            match remove_variables::main(&raw, env) {
                Ok(unfolded) => {
                    compute::bash(unfolded, env);
                    io::_print(&String::from("\n"));
                },
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
        for _ in previous_size..(1 + begin.len() + end.len()) {
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

fn read_all(fd: usize) -> String {
    let mut s = String::new();

    loop {
        let v = io::read_input(fd, 512);
        if v.len() == 0 {
            return s;
        }

        for i in v {
            s.push(i as char);
        }
    }
}