use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use ferr_os_librust::{io, syscall};

mod build_tree;
mod lexer;
use build_tree::command::{Command, Connector};

pub fn bash(string: String, env: &mut BTreeMap<String, String>) {
    match lexer::decompose(string) {
        Err(()) => io::_print(&String::from("Could not parse it\n")),
        Ok(vector) => match build_tree::build_tree(vector) {
            Err(()) => io::_print(&String::from("Could not parse formula\n")),
            Ok(command) => {
                unsafe { exec(command, env, false) };
            }
        },
    }
}

unsafe fn exec(command: Command, env: &mut BTreeMap<String, String>, background : bool) -> usize {
    match command {
        Command::Nothing => {
            0
        }
        Command::SimpleCommand(cmd) => {
            if cmd.cmd_line.len() >= 2 && cmd.cmd_line[1] == "=" {
                if cmd.cmd_line.len() > 2 {
                    env.insert(
                        String::from(&cmd.cmd_line[0]),
                        String::from(&cmd.cmd_line[2]),
                    );
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
                } else if prog_name.len() > 1
                    && prog_name.as_bytes()[0] == b'.'
                    && prog_name.as_bytes()[1] == b'/'
                {
                    if let Some(name) = env.get("PWD") {
                        if background {
                            let mut name = String::from(name);
                            for c in prog_name.bytes().skip(1) {
                                name.push(c as char);
                            }

                            let mut args = cmd.cmd_line;
                            if let Some(name) = env.get("PWD") {
                                args.push(String::from(name))
                            } else {
                                args.push(String::new())
                            }

                            run(&name, &args);
                            io::_print(&String::from("Program not found\n"));
                            syscall::exit(1)
                        } else {
                            let fd = syscall::open(&String::from("/dev/fifo"), io::OpenFlags::ORD | io::OpenFlags::OWR);
                            let id = syscall::fork();
                            if id == 0 {
                                syscall::dup2(io::STD_IN, fd);
                                syscall::close(fd);
                                let mut name = String::from(name);
                                for c in prog_name.bytes().skip(2) {
                                    name.push(c as char);
                                }
    
                                let mut args = cmd.cmd_line;
                                if let Some(name) = env.get("PWD") {
                                    args.push(String::from(name))
                                } else {
                                    args.push(String::new())
                                }
                                run(&name, &args);
                                io::_print(&String::from("Program not found\n"));
                                syscall::exit(1)
                            } else {
                                let mut data: [u8; 512] = [0; 512];
                                loop {
                                    let size = syscall::read(io::STD_IN, &mut data as *mut u8, 512);
                                    let mut kill = false;
                                    for i in 0..size {
                                        if data[i] == 12 {
                                            kill = true;
                                            io::_print(&String::from("Should kill user\n"));
                                        }
                                    }

                                    syscall::write(fd, &data as *const u8, size);
                                    syscall::sleep();
                                    if kill {
                                        syscall::close(fd);
                                        for i in 0..5 {
                                            syscall::sleep();
                                        }
                                        syscall::kill(id);
                                        return syscall::await_end(id);
                                    }
                                }
                            }
                        }
                    } else {
                        io::_print(&String::from(
                            "Variable PWD not defined (should not happen)\n",
                        ));
                        1
                    }
                } else {
                    if cmd.cmd_line[0] == "cd" {
                        if cmd.cmd_line.len() > 1 && cmd.cmd_line[1].len() > 0 {
                            let mut name = String::from(&cmd.cmd_line[1]);
                            if name.as_bytes()[name.len() - 1] != b'/' {
                                name.push('/');
                            }

                            if name.as_bytes()[0] == b'/' {
                                env.insert(String::from("PWD"), String::from(&name));
                            } else {
                                let mut pwd;
                                match env.get("PWD") {
                                    None => {pwd = String::from("/")},
                                    Some(p) => {pwd = String::from(p)},
                                }
                                pwd.push_str(&name);
                                env.insert(String::from("PWD"), String::from(&pwd));
                            }
                        } else {
                            env.insert(String::from("PWD"), String::from("/"));
                        }
                        0
                    } else if cmd.cmd_line[0] == "exit" {
                        if background {
                            if cmd.cmd_line.len() >= 2 {
                                let i = &cmd.cmd_line[1];
                                if i == "0" {
                                    syscall::exit(0)
                                } else {
                                    syscall::exit(1)
                                }
                            } else {
                                syscall::exit(0)
                            }
                        } else {
                            syscall::shutdown(0)
                        }
                    } else if let Some(name_list_raw) = env.get("PATH") {
                        if background {
                            let mut name_list = String::from(name_list_raw);
                            for name_raw in name_list.split(":") {
                                let mut name = String::from(name_raw);
                                if name.as_bytes()[name.len() - 1] != b'/' {
                                    name.push('/');
                                }
                                for c in prog_name.bytes() {
                                    name.push(c as char);
                                }

                                let mut args = Vec::new();
                                for a in cmd.cmd_line.iter() {
                                    args.push(String::from(a))
                                }

                                if let Some(name) = env.get("PWD") {
                                    args.push(String::from(name))
                                } else {
                                    args.push(String::new())
                                }
                                run(&name, &args);
                            }
                            io::_print(&String::from("Program not found\n"));
                            syscall::exit(1)
                        } else {
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

                                    let mut args = Vec::new();
                                    for a in cmd.cmd_line.iter() {
                                        args.push(String::from(a))
                                    }

                                    if let Some(name) = env.get("PWD") {
                                        args.push(String::from(name))
                                    } else {
                                        args.push(String::new())
                                    }
                                    run(&name, &args);
                                }
                                io::_print(&String::from("Program not found\n"));
                                syscall::exit(1)
                            } else {
                                syscall::await_end(id)
                            }
                        }
                    } else {
                        io::_print(&String::from("Variable PATH not defined\n"));
                        1
                    }
                }
            }
        }

        Command::Connection(cmd1, connect, cmd2) => {
            match connect {
                Connector::Seq => {
                    let output = exec(*cmd1, env, false);
                    if output == 0 {
                        exec(*cmd2, env, false)
                    } else {
                        output
                    }
                },

                Connector::And => {
                    io::_print(&String::from("& Not handled\n"));
                    1
                },

                Connector::Or => {
                    let output = exec(*cmd1, env, background);
                    if output != 0 {
                        exec(*cmd2, env, background)
                    } else {
                        output
                    }
                }

                Connector::Pipe => {
                    let fd = syscall::open(&String::from("/dev/fifo"), io::OpenFlags::ORD | io::OpenFlags::OWR);
                    if background {
                        if syscall::fork() == 0 {
                            syscall::dup2(io::STD_OUT, fd);
                            syscall::close(fd);
                            syscall::exit(exec(*cmd1, env, true))
                        } else {
                            syscall::dup2(io::STD_IN, fd);
                            syscall::close(fd);
                            syscall::exit(exec(*cmd2, env, true))
                        }
                    } else {
                        let proc_1 = syscall::fork();
                        if proc_1 == 0 {
                            syscall::dup2(io::STD_OUT, fd);
                            syscall::close(fd);
                            syscall::exit(exec(*cmd1, env, true))
                        } else {
                            match *cmd2 {
                                Command::Nothing => 0,
                                _ => {
                                    let proc_2 = syscall::fork();
                                    if proc_2 == 0 {
                                        syscall::dup2(io::STD_IN, fd);
                                        syscall::close(fd);
                                        syscall::exit(exec(*cmd2, env, true))
                                    } else {
                                        syscall::close(fd);
                                        wait_end(proc_1, proc_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Command::If(_, _, _) => {
            io::_print(&String::from("If nothing\n"));
            1
        } // Not implemented
    }
}


unsafe fn wait_end(proc_1: usize, proc_2: usize) -> usize {
    loop {
        let (i1, i2) = syscall::listen_proc(proc_1);
        if i1 == proc_1 {
            return syscall::await_end(proc_2)
        }

        let (i1, i2) = syscall::listen_proc(proc_2);
        if i1 == proc_2 {
            return syscall::await_end(proc_1)
        }

        syscall::sleep();

    }
}

unsafe fn run(path: &String, args: &Vec<String>) -> usize {
    let fd = syscall::open(path, io::OpenFlags::OXCUTE);
    if fd == usize::MAX {
        1
    } else {
        let first_char = io::read_input(fd, 1);
        match first_char.get(0) {
            None => 1,
            Some(c) => {
                if *c == 0x7f {
                    syscall::close(fd);
                    syscall::exec(path, args)
                } else {
                    io::_print(&String::from("Only ELF has been implemented\n"));
                    1
                }
            }
        }
    }
}