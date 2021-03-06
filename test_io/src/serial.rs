use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
    $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\nUser Info\n"));
    ($fmt:expr) => ($crate::print!(concat!("User Info: ", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!("User Info: ", $fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! warningln {
    () => ($crate::print!("\x1B[33mUser Warning \x1B[0m\n"));
    ($fmt:expr) => ($crate::print!(concat!("\x1B[33mUser Warning: ", $fmt, "\x1B[0m\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!("\x1B[33mUser Warning: ", $fmt, "\x1B[0m\n"), $($arg)*));
}

#[macro_export]
macro_rules! errorln {
    () => ($crate::print!("\x1B[91mUser ERROR \x1B[0m\n"));
    ($fmt:expr) => ($crate::print!(concat!("\x1B[91mUser ERROR: ", $fmt, "\x1B[0m\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!("\x1B[91mUser ERROR: ", $fmt, "\x1B[0m\n"), $($arg)*));
}

#[macro_export]
macro_rules! debug {
    () => ($crate::print!("\x1B[92mUser Debug \x1B[0m\n"));
    ($fmt:expr) => ($crate::print!(concat!("\x1B[92mUser Debug: ", $fmt, "\x1B[0m\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!("\x1B[92mUser Debug: ", $fmt, "\x1B[0m\n"), $($arg)*));
}

#[macro_export]
macro_rules! initdebugln {
    () => {
        $crate::print!("\n ===== User FerrOS debug interface =====\n")
    };
}
