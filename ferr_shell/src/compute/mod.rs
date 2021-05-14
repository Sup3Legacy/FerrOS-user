
use alloc::string::String;
use alloc::collections::BTreeMap;

use ferr_os_librust::io;

pub fn bash(string: String, env: &'static mut BTreeMap<String, String>) {
    io::_print(&String::from("bash not implemented"))
}