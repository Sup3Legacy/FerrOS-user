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
fn main(_args: Vec<String>) {
    io::_print(&String::from("\x1B[91m                    \n
               .#   ### ### ###   #               \n
           ##, #################(### (##          \n
           ############################/          \x1B[0mOS    : FerrOS\x1B[91m\n
        ########(###############(##########       \x1B[0mKernel: 0.0.0\x1B[91m\n
     #########################################    \x1B[0mShell : ferr_shell\x1B[91m\n
      ##########################(############     \x1B[0mUptime: 42 hours\x1B[91m\n
   ################\x1B[0m@@  \x1B[91m######\x1B[0m@@  \x1B[91n###############  \x1B[0mMemory: full\x1B[91m\n
  ,#####(#######(#\x1B[0m@@@@\x1B[91m  (####\x1B[0m@@@\x1B[91m  ######(#######( \x1B[0mMoto: Better than A0!\x1B[91m\n
 #################\x1B[0m@@@\x1B[91m   #####\x1B[0m@@\x1B[91m   ################\n
 #####%%%%######################(########%%%%#####\n
  ##### %%%###.%%%%#%%%%%%%%%%%%%%%% *####%%##### \n
    ####  %  ###(########   ####(#####(  %  ###   \n
      (##      ########### ##########       ##    \n
                #####  \x1B[0m@@   @\x1B[91m  #(###              \x1B[0m\n
                 @@      @  @@@                   \n
                 @@      @      @                 \n
                   @@@@@    @@@@                  \n
                                                  \n
NOTE: If the memory is full just download some more.\n\n"))
}
