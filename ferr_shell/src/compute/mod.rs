use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use ferr_os_librust::{io, syscall};

//mod build_tree;
pub mod lexer;
//use build_tree::command::{Command, Connector};
pub mod command;
use command::{Command, Connector};
pub mod parser;

pub fn bash(string: String, env: &mut BTreeMap<String, String>) {
    unsafe {
        syscall::debug(0, 0);
    }
    let mut lexbuf = lexer::Lexbuf::new(string);
    unsafe {
        syscall::debug(0, 1);
    }
    match parser::inputunit(lexer::token, &mut lexbuf) {
        Err(_) => io::_print(&String::from("parsing error\n")),
        Ok(command) => {
            unsafe { exec(command, env) };
        },
    }

    /*match lexer::decompose(string) {
        Err(()) => io::_print(&String::from("Could not parse it\n")),
        Ok(vector) => match build_tree::build_tree(vector) {
            Err(()) => io::_print(&String::from("Could not parse formula\n")),
            Ok(command) => {
                unsafe { exec(command, env, false) };
            }
        },
    }*/
}

unsafe fn exec(command: Command, env: &mut BTreeMap<String, String>) -> usize {
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
                            await_end_and_kill(id, fd)
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
                    } else if let Some(name_list_raw) = env.get("PATH") {
                        let fd = syscall::open(&String::from("/dev/fifo"), io::OpenFlags::OWR | io::OpenFlags::ORD);
                        let id = syscall::fork();
                        if id == 0 {
                            syscall::dup2(io::STD_IN, fd);
                            syscall::close(fd);
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
                            await_end_and_kill(id, fd)
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
                    let output = exec(*cmd1, env);
                    if output == 0 {
                        exec(*cmd2, env)
                    } else {
                        output
                    }
                },

                Connector::And => {
                    io::_print(&String::from("& Not handled\n"));
                    1
                },

                Connector::Or => {
                    let output = exec(*cmd1, env);
                    if output != 0 {
                        exec(*cmd2, env)
                    } else {
                        output
                    }
                }

                Connector::Pipe => {
                    let fd = syscall::open(&String::from("/dev/fifo"), io::OpenFlags::ORD | io::OpenFlags::OWR);
                    let fd2 = syscall::open(&String::from("/dev/fifo"), io::OpenFlags::ORD | io::OpenFlags::OWR);
                    let proc_1 = syscall::fork();
                    if proc_1 == 0 {
                        syscall::dup2(io::STD_OUT, fd);
                        syscall::close(fd);
                        syscall::dup2(io::STD_IN, fd2);
                        syscall::close(fd2);
                        syscall::exit(exec(*cmd1, env))
                    } else {
                        let proc_2 = syscall::fork();
                        if proc_2 == 0 {
                            syscall::close(fd2);

                            syscall::dup2(io::STD_IN, fd);
                            syscall::close(fd);
                            syscall::exit(exec(*cmd2, env))
                        } else {
                            syscall::close(fd);
                            await_end2_and_kill(proc_1, proc_2, fd2)
                        }
                    }
                }
            }
        }
        Command::If(cmd_if, cmd_then, cmd_else) => {
            let id = syscall::fork();
            let fd = syscall::open(&String::from("/dev/fifo"), io::OpenFlags::ORD | io::OpenFlags::OWR);
            if id == 0 {
                syscall::dup2(io::STD_IN, fd);
                syscall::close(fd);
                let v = exec(*cmd_if, env);
                syscall::exit(v)
            } else {
                let v = await_end_and_kill(id, fd);
                if v == 0 {
                    exec(*cmd_then, env)
                } else {
                    exec(*cmd_else, env)
                }
            }
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

unsafe fn await_end_and_kill(id: usize, fd: usize) -> usize {
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
            let (id2, v) = syscall::listen_proc(id);
            if id2 == id {
                return v;
            } else {
                syscall::kill(id);
                return syscall::await_end(id);
            }
        }

        let (id2, v) = syscall::listen_proc(id);
        if id2 == id {
            return v;
        }
    }
}

unsafe fn await_end2_and_kill(id1: usize, id2: usize, fd: usize) -> usize {
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
            let (id1bis, v) = syscall::listen_proc(id1);
            if id1bis == id1 {
                let (id2bis, v) = syscall::listen_proc(id2);
                if id2bis == id2 {
                    return v;
                } else {
                    syscall::kill(id2);
                    return syscall::await_end(id2);
                }
            } else {
                syscall::kill(id1);
                let (id2bis, v) = syscall::listen_proc(id2);
                if id2bis == id2 {
                    return syscall::await_end(id1);
                } else {
                    syscall::kill(id2);
                    return wait_end(id1, id2);
                }
            }
        }

        let (id1bis, v) = syscall::listen_proc(id1);
        if id1bis == id1 {
            return await_end_and_kill(id2, fd);
        }

        let (id2bis, v) = syscall::listen_proc(id2);
        if id2bis == id2 {
            return await_end_and_kill(id1, fd);
        }
    }
}