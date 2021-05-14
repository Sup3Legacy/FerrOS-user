
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::boxed::Box;

use ferr_os_librust::io;

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
                    exec(command, env);
                },
            }
        }
    }
}

fn exec(command: Command, env: &mut BTreeMap<String, String>) -> usize {
    match command {
        Command::Nothing => 0,
        Command::SimpleCommand(cmd) => {
            if cmd.cmd_line.len() >= 2 && cmd.cmd_line[1] == "=" {
                if cmd.cmd_line.len() > 2 {
                    env.insert(String::from(&cmd.cmd_line[0]), String::from(&cmd.cmd_line[2]));
                    /*match env.get("PWD") {
                        Some(v) => {
                            io::_print(v);
                            io::_print(&String::from("\n"));
                        },
                        None => io::_print(&String::from("PWD undefined\n")),
                    }*/
                    0
                } else {
                    //io::_print(&String::from("Wrong length\n"))
                    1
                }
            } else {
                1
            }
        },
        Command::Connection(cmd1, connect, cmd2) => {
            //io::_print(&String::from("Not handled\n"));
            1
        },
        Command::If(_, _, _) => 1, // Not implemented
    }
}