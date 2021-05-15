
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::boxed::Box;

use ferr_os_librust::{io, syscall};

mod build_tree;
mod lexer;
use build_tree::command::Command;


pub fn bash(string: String, env: &mut BTreeMap<String, String>) {
    match lexer::decompose(string) {
        Err(()) => {
            io::_print(&String::from("Could not parse it\n"))
        },
        Ok(vector) => {
            match build_tree::build_tree(vector) {
                Err(()) => io::_print(&String::from("Could not parse formula\n")),
                Ok(command) => {
                    unsafe {
                        exec(command, env)
                    };
                },
            }
        }
    }
}

unsafe fn exec(command: Command, env: &mut BTreeMap<String, String>) -> usize {
    match command {
        Command::Nothing => {
            io::_print(&String::from("Command nothing\n"));
            0
        },
        Command::SimpleCommand(cmd) => {
            io::_print(&String::from("Command Simple\n"));
            if cmd.cmd_line.len() >= 2 && cmd.cmd_line[1] == "=" {
                if cmd.cmd_line.len() > 2 {
                    env.insert(String::from(&cmd.cmd_line[0]), String::from(&cmd.cmd_line[2]));
                } else {
                    env.insert(String::from(&cmd.cmd_line[0]), String::from(""));
                }
                0
             } else if cmd.cmd_line.len() == 0 {
                0
            } else {
                let prog_name = &cmd.cmd_line[0];
                if prog_name.len() == 0 {
                    io::_print(&String::from("Should not happen compute->exec\n"));
                    1
                } else if prog_name.len() > 1 && prog_name.as_bytes()[0] == b'.' && prog_name.as_bytes()[1] == b'/' {
                    if let Some(name) = env.get("PWD") {
                        let id = syscall::fork();
                        if id == 0 {
                            let mut name = String::from(name);
                            for c in prog_name.bytes().skip(1) {
                                name.push(c as char);
                            }
                            
                            syscall::exec(name);
                            io::_print(&String::from("Program not found\n"));
                            syscall::exit(1)
                        } else {
                            syscall::await_end(id)
                        }
                    } else {
                        io::_print(&String::from("Variable PWD not defined (should not happen)\n"));
                        1
                    }
                } else {
                    if cmd.cmd_line[0] == "cd" {
                        if cmd.cmd_line.len() > 1 {
                            env.insert(String::from("PWD"), String::from(&cmd.cmd_line[1])) ;
                            0
                        } else {
                            1
                        }
                    } else if let Some(name_list_raw) = env.get("PATH") {
                        let id = syscall::fork();
                        if id == 0 {
                            let mut name_list = String::from(name_list_raw);
                            for name_raw in name_list.split(":") {
                                let mut name = String::from(name_raw);
                                if name.as_bytes()[name.len() - 1] != b'/' {
                                    name.push('/');
                                }
                                for c in prog_name.bytes() {
                                    name.push(c as char);
                                }
                                syscall::exec(name);
                            }
                            io::_print(&String::from("Program not found\n"));
                            loop {};
                            syscall::exit(1)
                        } else {
                            syscall::await_end(id)
                        }
                    } else {
                        io::_print(&String::from("Variable PATH not defined\n"));
                        1
                    }
                }
            }
        },
        Command::Connection(cmd1, connect, cmd2) => {
            io::_print(&String::from("Not handled\n"));
            1
        },
        Command::If(_, _, _) => {
            io::_print(&String::from("If nothing\n"));
            1
        }, // Not implemented
    }
}