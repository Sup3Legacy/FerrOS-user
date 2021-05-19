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

/* Without colors.
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
        "  ,#####(#######(#@@@@  (####@@@  ######(#######(   | Moto  : Better than A0!"
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
*/

// With colors!
#[inline(never)]
fn main(_args: Vec<String>) {
    print_lines!(
        "\x1B[\x05m"
        "\x1B[\x05m               .#   ### ### ###   #               "
        "\x1B[\x05m           ##, #################(### (##          "
        "\x1B[\x05m           ############################/          \x1B[\x10m  | OS    : FerrOS\x1B[\x05m"
        "\x1B[\x05m        ########(###############(##########       \x1B[\x10m  | Kernel: 0.0.0\x1B[\x05m"
        "\x1B[\x05m     #########################################    \x1B[\x10m  | Shell : ferr_shell\x1B[\x05m"
        "\x1B[\x05m      ##########################(############     \x1B[\x10m  | Uptime: 42 hours\x1B[\x05m"
        "\x1B[\x05m   ################\x1B[\x10m@@  \x1B[\x05m######\x1B[\x10m@@  \x1B[\x05m###############  \x1B[\x10m  | Memory: full\x1B[\x05m"
        "\x1B[\x05m  ,#####(#######(#\x1B[\x10m@@@@\x1B[\x05m  (####\x1B[\x10m@@@\x1B[\x05m  ######(#######( \x1B[\x10m  | Moto  : Better than A0!\x1B[\x05m"
        "\x1B[\x05m #################\x1B[\x10m@@@\x1B[\x05m   #####\x1B[\x10m@@\x1B[\x05m   ################"
        "\x1B[\x05m #####%%%%######################(########%%%%#####"
        "\x1B[\x05m  ##### %%%###.%%%%#%%%%%%%%%%%%%%%% *####%%##### "
        "\x1B[\x05m    ####  %  ###(########   ####(#####(  %  ###   "
        "\x1B[\x05m      (##      ########### ##########       ##    "
        "\x1B[\x05m                #####  \x1B[\x10m@@   @\x1B[\x05m  #(###              \x1B[\x10m"
        "                 @@      @  @@@                   "
        "                 @@      @      @                 "
        "                   @@@@@    @@@@                  "
        ""
        "NOTE: If the memory is full just download some more."
        ""
    );
}
