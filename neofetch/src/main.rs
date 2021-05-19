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
        "        ########(###############(##########         | Kernel: 0.1.0"
        "     #########################################      | Shell : ferr_shell"
        "      ##########################(############       | Uptime: 42 hours"
        "   ################@@  ######@@  ###############    | Memory: full"
        "  ,#####(#######(#@@@@  (####@@@  ######(#######(   | Motto : Better than A0!"
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
        "               .#   ### ### ###   #               "
        "           ##, #################(### (##          "
        "           ############################/          \x1B[\x0Fm  | OS    : \x1B[\x06mFerrOS\x1B[\x05m"
        "        ########(###############(##########       \x1B[\x0Fm  | Kernel: \x1B[\x06m0.1.0\x1B[\x05m"
        "     #########################################    \x1B[\x0Fm  | Shell : \x1B[\x06mferr_shell\x1B[\x05m"
        "      ##########################(############     \x1B[\x0Fm  | Uptime: \x1B[\x06m42 hours\x1B[\x05m"
        "   ################\x1B[\x10m@@  \x1B[\x05m######\x1B[\x10m@@  \x1B[\x05m###############  \x1B[\x0Fm  | Memory: \x1B[\x06mFull\x1B[\x05m"
        "  ,#####(#######(#\x1B[\x10m@@@@\x1B[\x05m  (####\x1B[\x10m@@@\x1B[\x05m  ######(#######( \x1B[\x0Fm  | Motto : \x1B[\x06mBetter than A0!\x1B[\x05m"
        " #################\x1B[\x10m@@@\x1B[\x05m   #####\x1B[\x10m@@\x1B[\x05m   ################\x1B[\x0Fm  | Theme : \x1B[\x06mUlmite\x1B[\x05m"
        " #####%%%%######################(########%%%%#####    \x1B[\x15m \x1B[\x16m \x1B[\x17m \x1B[\x18m \x1B[\x19m \x1B[\x1Am \x1B[\x1Bm \x1B[\x1Cm \x1B[\x15m"
        "  ##### %%%###.%%%%#%%%%%%%%%%%%%%%% *####%%#####     \x1B[\x1Dm \x1B[\x1Em \x1B[\x1Fm \x1B[\x20m \x1B[\x21m \x1B[\x22m \x1B[\x23m \x1B[\x24m \x1B[\x15m"
        "    ####  %  ###(########   ####(#####(  %  ###   "
        "      (##      ########### ##\x1B[\x08m@@@\x1B[\x05m#####       ##    "
        "                #####  \x1B[\x08m@@   @\x1B[\x05m  #(###              \x1B[\x08m"
        "                 @@      @  @@@                   "
        "                 @@      @      @                 "
        "                   @@@@@    @@@@                  "
        ""
        "\x1B[\x03mNOTE:\x1B[\x0Fm If the memory is full just download some more.\x1B[\x10m"
        ""
    );
}
