use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use ferr_os_librust::{io, syscall};

//mod build_tree;
pub mod lexer;
//use build_tree::command::{Command, Connector};
pub mod command;
use command::{Command, Connector, Redirect};
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
                    let mut pwd;
                    match env.get("PWD") {
                        Some(n) => pwd = String::from(n),
                        None => pwd = String::new(),
                    }
                    let id;
                    let fd = syscall::open(&String::from("/dev/fifo"), io::OpenFlags::ORD | io::OpenFlags::OWR);
                    if cmd.cmd_bg {
                        id = 0;
                    } else {
                        id = syscall::fork();
                    }
                    if id == 0 {
                        if !cmd.cmd_bg {
                            syscall::dup2(io::STD_IN, fd);
                        }
                        syscall::close(fd);
                        do_redirects(&cmd.cmd_redirects, &pwd);

                        let mut name = String::from(&pwd);
                        for c in prog_name.bytes().skip(2) {
                            name.push(c as char);
                        }
                        
                        let mut args = cmd.cmd_line;
                        args.push(String::from(&pwd));

                        run(&name, &args);
                        io::_print(&String::from("Program not found\n"));
                        syscall::exit(1)
                    } else {
                        await_end_and_kill(id, fd)
                    }
                } else if prog_name.as_bytes()[0] == b'/' {
                    let mut pwd;
                    match env.get("PWD") {
                        Some(n) => pwd = String::from(n),
                        None => pwd = String::new(),
                    }
                    let id;
                    let fd = syscall::open(&String::from("/dev/fifo"), io::OpenFlags::ORD | io::OpenFlags::OWR);
                    if cmd.cmd_bg {
                        id = 0;
                    } else {
                        id = syscall::fork();
                    }
                    if id == 0 {
                        if !cmd.cmd_bg {
                            syscall::dup2(io::STD_IN, fd);
                        }
                        syscall::close(fd);
                        do_redirects(&cmd.cmd_redirects, &pwd);
                        let name = String::from(prog_name);
                        let mut args = cmd.cmd_line;
                        args.push(String::from(&pwd));

                        run(&name, &args);
                        io::_print(&String::from("Program not found\n"));
                        syscall::exit(1)
                    } else {
                        await_end_and_kill(id, fd)
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
                        let id;
                        if cmd.cmd_bg {
                            id = 0
                        } else {
                            id = syscall::fork()
                        };

                        if id == 0 {
                            if !cmd.cmd_bg {
                                syscall::dup2(io::STD_IN, fd);
                            }
                            syscall::close(fd);
                            let pwd;
                            if let Some(name) = env.get("PWD") {
                                pwd = String::from(name);
                            } else {
                                pwd = String::new();
                            }
                            do_redirects(&cmd.cmd_redirects, &pwd);

                            let name_list = String::from(name_list_raw);
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

                                args.push(String::from(&pwd));
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
                    let fd = syscall::open(&String::from("/dev/fifo"), io::OpenFlags::ORD | io::OpenFlags::OWR);
                    let proc_1 = syscall::fork();
                    if proc_1 == 0 {
                        syscall::dup2(io::STD_IN, fd);
                        syscall::close(fd);
                        syscall::exit(exec(*cmd1, env))
                    } else {
                        let proc_2 = syscall::fork();
                        if proc_2 == 0 {
                            syscall::dup2(io::STD_IN, fd);
                            syscall::close(fd);
                            syscall::exit(exec(*cmd2, env))
                        } else {
                            syscall::close(fd);
                            await_end2_and_kill(proc_1, proc_2, fd)
                        }
                    }
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
        let (i1, _) = syscall::listen_proc(proc_1);
        if i1 == proc_1 {
            return syscall::await_end(proc_2)
        }

        let (i1, _) = syscall::listen_proc(proc_2);
        if i1 == proc_2 {
            return syscall::await_end(proc_1)
        }

        syscall::sleep();

    }
}

unsafe fn do_redirects(redirects: &Vec<Redirect>, pwd: &String) {
    for r in redirects.iter() {
        let file_name;
        let fd_target;
        let rights;
        match r {
            Redirect::Input(s) => {
                file_name = s;
                fd_target = io::STD_IN;
                rights = io::OpenFlags::ORD;
            },
            Redirect::Output(s) => {
                file_name = s;
                fd_target = io::STD_OUT;
                rights = io::OpenFlags::OWR | io::OpenFlags::OCREAT;
            },
            Redirect::OutputAppend(s) => {
                file_name = s;
                fd_target = io::STD_OUT;
                rights = io::OpenFlags::OWR | io::OpenFlags::OCREAT | io::OpenFlags::OAPPEND;
                syscall::debug(0, 2);
            }
        };
        if file_name.len() > 0 {
            let mut file;
            if file_name.as_bytes()[0] == b'/' {
                file = String::from(file_name);
            } else {
                if file_name.len() > 1 && &file_name.as_bytes()[0..1] == "./".as_bytes() {
                    file = String::from(pwd);
                    for b in file_name.bytes().skip(2) {
                        file.push(b as char);
                    }
                } else {
                    file = String::from(pwd);
                    for b in file_name.bytes() {
                        file.push(b as char);
                    }
                }
            }
            let fd = syscall::open(&file, rights);
            syscall::dup2(fd_target, fd);
            syscall::close(fd);
        } else {
            syscall::exit(1);
        }
    } 
}

unsafe fn run(path: &String, args: &Vec<String>) -> usize {
    let fd = syscall::open(path, io::OpenFlags::OXCUTE);
    if fd == usize::MAX {
        1
    } else {
        let start = io::read_input(fd, 512);
        syscall::close(fd);
        if start.len() == 0 {
            1
        } else if start.len() > 4 && &start[0..4] == "\x7FELF".as_bytes() {
            syscall::exec(path, args)
        } else if start.len() > 4 && &start[0..3] == "#!/".as_bytes() {
            let mut name = String::from(start[2] as char);
            for i in 3..start.len() {
                if start[i] == b'\n' {
                    break
                } else {
                    name.push(start[i] as char);
                }
            }
            launch_elf(&name, args)
        } else {
            io::_print(&String::from("\nOnly ELF has been implemented\n"));
            syscall::exit(1)
        }
    }
}

unsafe fn launch_elf(path: &String, args: &Vec<String>) -> usize {
    let fd = syscall::open(path, io::OpenFlags::OXCUTE);
    if fd == usize::MAX {
        syscall::exit(1)
    } else {
        let start = io::read_input(fd, 4);
        syscall::close(fd);
        if start.len() < 4 {
            syscall::exit(1)
        } else if &start == "\x7ELF".as_bytes() {
            syscall::exec(path, args)
        } else {
            syscall::exit(2)
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
            for _ in 0..5 {
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
            syscall::close(fd);
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
            for _ in 0..5 {
                syscall::sleep();
            }
            let (id1bis, _) = syscall::listen_proc(id1);
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
                let (id2bis, _) = syscall::listen_proc(id2);
                if id2bis == id2 {
                    return syscall::await_end(id1);
                } else {
                    syscall::kill(id2);
                    return wait_end(id1, id2);
                }
            }
        }

        let (id1bis, _) = syscall::listen_proc(id1);
        if id1bis == id1 {
            return await_end_and_kill(id2, fd);
        }

        let (id2bis, _) = syscall::listen_proc(id2);
        if id2bis == id2 {
            return await_end_and_kill(id1, fd);
        }
    }
}