#![no_std]
#![feature(start)]
#![feature(asm)]
#![no_main]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

static  _A: &'static str = "Hello, World kojqsdioqjsdioqhjsujdchcviushiuxcnQIUDHSSDUIHNBQsiujdhb quiSH DUICQHBIUDChn iqjhdiCJQHNDJKQSHCDKJQSHDKJC H";

#[no_mangle]
pub extern "C" fn _start() {
    init();
    let mut port = Port::new(0xa2);
    unsafe {
        port.write(0xA2_u16);
    }
    main(_A);
    //panic!();
}

fn init() {
    unsafe { asm!("mov rax, 2", "int 80h",) }
}

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    //unsafe { asm!("mov rax, 2", "int 80h",) }
    loop {}
}

#[inline(never)]
fn main(a: &'static str) {
    let mut port = Port::new(0xa2);
    for e in a.as_bytes() {
        unsafe {
            port.write(*e);
        }
    }
}
