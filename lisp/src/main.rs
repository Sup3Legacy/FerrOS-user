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
//extern crate combine;

use alloc::collections::btree_map::BTreeMap;
use alloc::fmt;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use core::ops::Add;

struct IFunc {
    fun: dyn Fn(Vec<LispVal>) -> Option<(LispVal, EnvCtx)>,
}

type ParserResult<'a, A> = Option<(&'a str, &'a A)>;

type Parser<'a, A> = dyn Fn(&str) -> ParserResult<'a, A>;

/*
// Rust ne sait pas faire des traits pour les types non définits dans la même crate.
struct ParserWrap<'a, A>(Parser<'a, A>);
impl<'a, A> ParserWrap<'a, A> {
    fn unwrap(&self) -> Parser<'a, A> {
        match self {
            ParserWrap(p) => p,
        }
    }
}

impl<'a, A> Add for &ParserWrap<'a, A> {
    type Output = Self;

    fn add(self, first_p: Self) -> Self {
        // |s: &str| apply_if_none<'a, A>(first_p(s), self, s)
        |s: &str| (first_p.unwrap())(s).or(self(s))
    }
}

fn apply_if_none<'a, A>(
    o: ParserResult<'a, A>,
    f: &Parser<'a, A>,
    s: &str,
) -> &'a ParserResult<'a, A> {
    &o.or(f(s))
}

fn apply_if_none<'a, A>(o: ParserResult<'a, A>, f: &Parser<'a, A>) -> &'a Parser<'a, A> {
    match o {
        None => &move |s| f(s),
        _ => &p_from_const(o),
    }
}

fn p_from_const<'a, A>(c: ParserResult<'a, A>) -> impl Fn(&str) -> ParserResult<'a, A> {
    move |_s| c
}
*/

macro_rules! alt {
    ($s: expr ; $p: ident) => {
        $p($s)
    };

    ($s: expr ; $p: ident | $( $tail: ident )|* ) => {
        (alt! { $s ; $( $tail )|* }).or($p($s))
    };
}

fn parse(s: &str) -> ParserResult<LispVal> {
    alt! { s ; lisp_nil | lisp_nil | lisp_bool }
}

fn string_p<'a>(s_p: &'a str, s: &'a str) -> ParserResult<'a, str> {
    match s.strip_prefix(s_p) {
        None => None,
        Some(tail) => Some((tail, s_p)),
    }
}

/*
macro_rules! let_char_p {
    ( $( $name: ident $char: literal );* ) => {
        let curried_string_p = |s_p| move |s| string_p(s_p, s);
        $(
            let $name = curried_string_p($char);
        )*
    }
}
*/

macro_rules! alt_string_p {
    ( $s: expr ; $( $name: ident $string: literal )|* ) => {
        let curried_string_p = |s_p| move |s| string_p(s_p, s);
        $(
            let $name = curried_string_p("$string");
        )*
        alt! { $s ; $( $name )|* }
    };
}

fn digit_p(s: &str) -> ParserResult<str> {
    /*
    let_char_p! {
        zero_p "0"; one_p "1"; two_p "2"; three_p "3"; four_p "4";
        five_p "5"; six_p "6"; seven_p "7"; eight_p "8"; nine_p "9"
    };
    */
    /*
    let curried_string_p = |s_p| move |s| string_p(s_p, s);
    let zero_p = curried_string_p("0");
    let one_p = curried_string_p("1");
    let two_p = curried_string_p("2");
    let three_p = curried_string_p("3");
    let four_p = curried_string_p("4");
    let five_p = curried_string_p("5");
    let six_p = curried_string_p("6");
    let seven_p = curried_string_p("7");
    let eight_p = curried_string_p("8");
    let nine_p = curried_string_p("9");
    */
    /*
    alt! {
        s ; zero_p | one_p | two_p | three_p | four_p
        | five_p | six_p | seven_p | eight_p | nine_p
    }
    */
    alt_string_p! {
        s ; zero "0" | one "1" | two "2" | three "3" | four "4"
        | five "5" | six "6" | seven "7" | eight "8" | nine "9"
    }
}

// TODO penetrate Maybe
fn lisp_nil(s: &str) -> ParserResult<LispVal> {
    string_p("Nil", s).map(|(tail, _)| (tail, &LispVal::Nil))
}

fn true_p(s: &str) -> ParserResult<LispVal> {
    string_p("#t", s).map(|(tail, _)| (tail, &LispVal::Bool(true)))
}

fn false_p(s: &str) -> ParserResult<LispVal> {
    string_p("#f", s).map(|(tail, _)| (tail, &LispVal::Bool(false)))
}

fn lisp_bool(s: &str) -> ParserResult<LispVal> {
    alt! { s ; false_p | true_p }
}

/*
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

type EnvCtx<'a> = BTreeMap<&'a str, LispVal<'a>>;

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
