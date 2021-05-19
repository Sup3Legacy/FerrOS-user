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
    io::_print(&String::from("                    \n
               .#   ### ### ###   #               \n
           ##, #################(### (##          \n
           ############################/          OS    : FerrOS\n
        ########(###############(##########       Kernel: 0.0.0\n
     #########################################    Shell : ferr_shell\n
      ##########################(############     Uptime: 42 hours\n
   ################  @@######% @@###############  Memory: full\n
  ,#####(#######(#@  @@@(###@( @@@######(#######( Moto: Better than A0!\n
 #################@@@@@@#####@@@@@################\n
 #####%%%%######################(########%%%%#####\n
  ##### %%%###.%%%%#%%%%%%%%%%%%%%%% *####%%##### \n
    ####  %  ###(########   ####(#####(  %  ###   \n
      (##      ########### ##########       ##    \n
                #####  @@   @  #(###              \n
                 @@      @  @@@                   \n
                 @@      @      @                 \n
                   @@@@@    @@@@                  \n
                                                  \n
NOTE: If the memory is full just download some more.\n\n"))
}
