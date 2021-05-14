use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use ferr_os_librust::io;

mod command;
mod lexer;
mod parser;

pub fn bash(string: String, env: &mut BTreeMap<String, String>) {
    let mut lexbuf = lexer::Lexbuf::new(string);
    let code = parser::inputunit(lexer::token, &mut lexbuf);
}
