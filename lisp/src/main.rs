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

use core::panic::PanicInfo;
use x86_64::VirtAddr;
mod serial;

extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::fmt;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

struct IFunc {
    fun: dyn Fn(Vec<LispVal>) -> Option<(LispVal, EnvCtx)>,
}

type EnvCtx = BTreeMap<String, LispVal>;

enum LispVal {
    Atom(String),
    List(Vec<LispVal>),
    Number(isize),
    String(String),
    Fun(Box<IFunc>),
    Lambda(Box<IFunc>, EnvCtx),
    Nil,
    Bool(bool),
}

impl fmt::Display for LispVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            LispVal::Atom(s) => write!(f, "{}", s),
            LispVal::List(l) => {
                let mut s = String::new();

                l.into_iter().for_each(|e| {
                    s.push_str((format!(" {}", e)).as_str());
                });
                write!(f, "({} )", s)
            }
            LispVal::Number(n) => write!(f, "{}", n),
            LispVal::String(s) => write!(f, "{}", s),
            LispVal::Fun(_) => write!(f, "(internal function)"),
            LispVal::Lambda(_, _) => write!(f, "(lambda function)"),
            LispVal::Nil => write!(f, "Nil"),
            LispVal::Bool(true) => write!(f, "#t"),
            LispVal::Bool(false) => write!(f, "#f"),
        }
    }
}

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64) {
    syscall(20, 1, 0, 0);

    set_screen_size(19, 79);
    set_screen_position(1, 1);
    ferr_os_librust::allocator::init(heap_address, heap_size);
    let mut a = String::new();
    a.push('a');
    if fork() == 0 {
        exec((&a as *const String) as u64)
    }
    //print(&a);
    //println!("Whelp!");
    main();
}

#[inline(never)]
fn main() {
    let read_buffer = [0_u8; 256];
    let mut buffer = [0_u8; 256];

    loop {
        let address = VirtAddr::from_ptr(read_buffer.as_ptr() as *mut u8);
        let length = syscall(0, 0, address.as_u64(), 256);
        let write_length = ferr_os_librust::interfaces::keyboard::decode_buffer(
            &read_buffer[..],
            &mut buffer[..],
            length,
        );
        print_buffer(&buffer[..], write_length);
        halt();
    }
}

fn fork() -> u64 {
    syscall(5, 0, 0, 0) as u64
}

fn exec(s: u64) {
    syscall(6, s, 0, 0);
}

fn set_screen_size(height: u64, width: u64) {
    syscall(11, height, width, 0);
}

fn set_screen_position(height: u64, width: u64) {
    syscall(12, height, width, 0);
}

fn print_buffer(buffer: &[u8], size: usize) {
    let mut index = 0_usize;
    let mut t: [u8; 256] = [0; 256];

    for c in 0..size {
        //syscall(20, index as u64, c as u64, 0);
        t[c] = buffer[c];
    }
    let data_addr = VirtAddr::from_ptr(&t as *const u8);
    syscall(1, 1, data_addr.as_u64(), size as u64);
}

fn print(a: &String) {
    let mut t: [u8; 128] = [0; 128];
    //syscall(20, 42, 0);
    let mut index = 0_usize;

    for c in a.bytes() {
        //syscall(20, index as u64, c as u64, 0);
        t[index] = c;
        index += 1;
        if index == 128 {
            t[index - 1] = 0; // We put a guard
            break;
        }
    }
    let data_addr = VirtAddr::from_ptr(&t as *const u8);
    syscall(1, 1, data_addr.as_u64(), index as u64);
}

pub fn halt() {
    syscall(8, 0, 0, 0);
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
