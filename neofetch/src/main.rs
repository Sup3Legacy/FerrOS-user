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

/*
#[inline(never)]
fn main(_args: Vec<String>) {
    io::_print(&String::from("\x1B[91m                    
               .#   ### ### ###   #               
           ##, #################(### (##          
           ############################/          \x1B[0mOS    : FerrOS\x1B[91m
        ########(###############(##########       \x1B[0mKernel: 0.0.0\x1B[91m
     #########################################    \x1B[0mShell : ferr_shell\x1B[91m
      ##########################(############     \x1B[0mUptime: 42 hours\x1B[91m
   ################\x1B[0m@@  \x1B[91m######\x1B[0m@@  \x1B[91m###############  \x1B[0mMemory: full\x1B[91m
  ,#####(#######(#\x1B[0m@@@@\x1B[91m  (####\x1B[0m@@@\x1B[91m  ######(#######( \x1B[0mMoto: Better than A0!\x1B[91m
 #################\x1B[0m@@@\x1B[91m   #####\x1B[0m@@\x1B[91m   ################
 #####%%%%######################(########%%%%#####
  ##### %%%###.%%%%#%%%%%%%%%%%%%%%% *####%%##### 
    ####  %  ###(########   ####(#####(  %  ###   
      (##      ########### ##########       ##    
                #####  \x1B[0m@@   @\x1B[91m  #(###              \x1B[0m
                 @@      @  @@@                   
                 @@      @      @                 
                   @@@@@    @@@@                  
                                                  
NOTE: If the memory is full just download some more.\n\n"))
}
*/

// Macros are evaluated at compile time.
// So it is faster than a loop.
// This macro is needed to bypass the buffer limit.
macro_rules! print_lines {
    ( $( $l: expr )* ) => {
        $(
            {
                io::_print(&String::from($l));
                io::_print(&String::from("\n"));
            }
        )*
    };
}

#[inline(never)]
fn main(_args: Vec<String>) {
    print_lines!(
        ""
        "               .#   ### ### ###   #                 "
        "           ##, #################(### (##            "
        "           ############################/            | OS    : FerrOS"
        "        ########(###############(##########         | Kernel: 0.0.0"
        "     #########################################      | Shell : ferr_shell"
        "      ##########################(############       | Uptime: 42 hours"
        "   ################@@  ######@@  ###############    | Memory: full"
        "  ,#####(#######(#@@@@  (####@@@  ######(#######(   | Moto: Better than A0!"
        " #################@@@   #####@@   ################  "
        " #####%%%%######################(########%%%%#####  "
        "  ##### %%%###.%%%%#%%%%%%%%%%%%%%%% *####%%#####   "
        "    ####  %  ###(########   ####(#####(  %  ###     "
        "      (##      ########### ##########       ##      "
        "                #####  @@   @  #(###                "
        "                 @@      @  @@@                     "
        "                 @@      @      @                   "
        "                   @@@@@    @@@@                    "
        ""
        "NOTE: If the memory is full just download some more."
        ""
    );
}
