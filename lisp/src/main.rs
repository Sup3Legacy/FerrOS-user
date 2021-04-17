// #![doc(html_favicon_url = "https://github.com/Sup3Legacy/FerrOS/blob/main/docs/images/FerrOS.ico")]
// #![doc(html_logo_url = "https://github.com/Sup3Legacy/FerrOS/blob/main/FerrOS.png")]

//! A minimal Lisp interpreter for [`FerrOS`](https://github.com/Sup3Legacy/FerrOS).

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

use x86_64::VirtAddr;
mod serial;

extern crate alloc;

use alloc::collections::btree_map::BTreeMap;
use alloc::fmt;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Define the AST.

/// We use references in oder to have fixed sized objects.
enum LispVal<'a> {
    Atom(&'a str),
    List(Vec<LispVal<'a>>),
    Number(isize),
    String(&'a str),
    Fun(&'a IFunc),
    Lambda(&'a IFunc, EnvCtx<'a>),
    Nil,
    Bool(bool),
}

/// Environment for evaluating expressions.
type EnvCtx<'a> = BTreeMap<&'a str, LispVal<'a>>;

/// Represents functions.
/// The context is needed for scoping.
struct IFunc {
    fun: dyn Fn(Vec<LispVal>) -> Option<(LispVal, EnvCtx)>,
}

/// Define a printer for [LispVal].
impl<'a> fmt::Display for LispVal<'a> {
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

// Define the parser.

// Parser generic return type.
type ParserResult<'a, A> = Option<(&'a str, &'a A)>;

// Type for generic Parsers.
type Parser<'a, A> = dyn Fn(&str) -> ParserResult<'a, A>;

// Combine parsers.
macro_rules! alt {
    ($s: expr ; $p: ident) => {
        $p($s)
    };

    ($s: expr ; $p: ident | $( $tail: ident )|* ) => {
        (alt! { $s ; $( $tail )|* }).or($p($s))
    };
}

/// Parses a given string.
fn string_p<'a>(s_p: &'a str, s: &'a str) -> ParserResult<'a, str> {
    match s.strip_prefix(s_p) {
        None => None,
        Some(tail) => Some((tail, s_p)),
    }
}

/// Parses [LispVal::Nil].
fn lisp_nil(s: &str) -> ParserResult<LispVal> {
    string_p("Nil", s).map(|(tail, _)| (tail, &LispVal::Nil))
}

// Booleans.

/// Parses `#t`.
fn true_p(s: &str) -> ParserResult<LispVal> {
    string_p("#t", s).map(|(tail, _)| (tail, &LispVal::Bool(true)))
}

/// Parses `#f`.
fn false_p(s: &str) -> ParserResult<LispVal> {
    string_p("#f", s).map(|(tail, _)| (tail, &LispVal::Bool(false)))
}

/// Parses [LispVal::Bool].
fn lisp_bool(s: &str) -> ParserResult<LispVal> {
    alt! { s ; false_p | true_p }
}

// Numbers.

// Create string parsers and combine them.
macro_rules! alt_string_p {
    ( $s: expr ; $( $name: ident $string: literal )|* ) => {
        let curried_string_p = |s_p| move |s| string_p(s_p, s);
        $(
            let $name = curried_string_p($string);
        )*
        alt! { $s ; $( $name )|* }
    };
}

/// Parses a digit as a [str].
fn digit_p(s: &str) -> ParserResult<str> {
    alt_string_p! {
        s ; zero "0" | one "1" | two "2" | three "3" | four "4"
        | five "5" | six "6" | seven "7" | eight "8" | nine "9"
    }
}

/// Combine all of the parsers into one to parse a [LispVal].
fn parse(s: &str) -> ParserResult<LispVal> {
    alt! { s ; lisp_nil | lisp_nil | lisp_bool }
}

/// Repeats a parser as much as possible and folds the results.
fn repeat<'a, A>(
    p: &Parser<'a, A>,
    s: &str,
    combine: &(dyn Fn(&A, ParserResult<'a, A>) -> ParserResult<'a, A>),
) -> ParserResult<'a, A> {
    p(s).and_then(|actual_res| combine(actual_res.1, repeat(p, actual_res.0, combine)))
}

/*
/// TODO
fn positive_number_combine<'a>(
    digit: &usize,
    p: &ParserResult<'a, usize>,
) -> ParserResult<'a, usize> {
    p.map(|tail_res| {
        (
            tail_res.0,
            &((format!("{}{}", digit, tail_res.1))
                .parse::<usize>()
                .ok()
                .unwrap()),
        )
    }) // TODO + digit
}
*/

/*
/// Parses a positive number.
fn positive_number_p(s: &str) -> ParserResult<str> {
    repeat(&digit_p, s, &concat_combine)
}
*/

/*
TODO

Comments

fn lisp_quote(s: &str) -> ParserResult<LispVal> {
}
fn lisp_s_expr(s: &str) -> ParserResult<LispVal> {
}
fn lisp_string(s: &str) -> ParserResult<LispVal> {
}
fn lisp_atom(s: &str) -> ParserResult<LispVal> {
}
fn lisp_number(s: &str) -> ParserResult<LispVal> {
}
fn lisp_lisp(s: &str) -> ParserResult<LispVal> {
}
fn lisp_lambda(s: &str) -> ParserResult<LispVal> {
}
fn lisp_fun(s: &str) -> ParserResult<LispVal> {
}
*/

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
