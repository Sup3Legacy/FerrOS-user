
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub mod command;


pub fn build_tree(vector : Vec<String>) -> Result<command::Command, ()> {
    let mut out = None;
    let mut line = Vec::new();
    for lexem in vector.iter() {
        if lexem == "&&" {
            let command2 = match build_tree_and(line) {
                Err(()) => return Err(()),
                Ok(command2) => command2
            };

            out = Some(match out {
                None => command2,
                Some(command) =>
                    command::Command::Connection(
                        Box::new(command),
                        command::Connector::Seq,
                        Box::new(command2)
                    ),
            });
            line = Vec::new();
        } else {
            line.push(String::from(lexem))
        }
    }
    match out {
        None => build_tree_and(line),
        Some(command) => 
            match build_tree_and(line) {
                Err(()) => Err(()),
                Ok(command2) =>
                    Ok(command::Command::Connection(
                        Box::new(command),
                        command::Connector::Seq,
                        Box::new(command2)
                    )),
            }
    }    
}

fn build_tree_and(vector : Vec<String>) -> Result<command::Command, ()> {
    let mut out = None;
    let mut line = Vec::new();
    for lexem in vector.iter() {
        if lexem == "&" {
            let command2 = match build_tree_or(line) {
                Err(()) => return Err(()),
                Ok(command2) => command2
            };

            out = Some(match out {
                None => command2,
                Some(command) => 
                    command::Command::Connection(
                        Box::new(command),
                        command::Connector::And,
                        Box::new(command2)
                    ),
            });
            line = Vec::new();
        } else {
            line.push(String::from(lexem))
        }
    }
    match out {
        None => build_tree_or(line),
        Some(command) => 
            match build_tree_or(line) {
                Err(()) => Err(()),
                Ok(command2) =>
                    Ok(command::Command::Connection(
                        Box::new(command),
                        command::Connector::And,
                        Box::new(command2)
                    )),
            }
    }    
}

fn build_tree_or(vector : Vec<String>) -> Result<command::Command, ()> {
    let mut out = None;
    let mut line = Vec::new();
    for lexem in vector.iter() {
        if lexem == "||" {
            let command2 = match build_tree_pipe(line) {
                Err(()) => return Err(()),
                Ok(command2) => command2
            };

            out = Some(match out {
                None => command2,
                Some(command) =>
                    command::Command::Connection(
                        Box::new(command),
                        command::Connector::Or,
                        Box::new(command2)
                    ),
            });
            line = Vec::new();
        } else {
            line.push(String::from(lexem))
        }
    }
    match out {
        None => build_tree_pipe(line),
        Some(command) => 
            match build_tree_pipe(line) {
                Err(()) => Err(()),
                Ok(command2) =>
                    Ok(command::Command::Connection(
                        Box::new(command),
                        command::Connector::Or,
                        Box::new(command2)
                    )),
            }
    }    
}

fn build_tree_pipe(vector: Vec<String>) -> Result<command::Command, ()> {
    let mut out = None;
    let mut line = Vec::new();
    for lexem in vector.iter() {
        if lexem == "|" {
            let command2 = match build_tree_low(line) {
                Err(()) => return Err(()),
                Ok(command2) => command2
            };

            out = Some(match out {
                None => command2,
                Some(command) => 
                    command::Command::Connection(
                        Box::new(command),
                        command::Connector::Pipe,
                        Box::new(command2)
                    ),
            });
            line = Vec::new();
        } else {
            line.push(String::from(lexem))
        }
    }
    match out {
        None => build_tree_low(line),
        Some(command) => 
            match build_tree_low(line) {
                Err(()) => Err(()),
                Ok(command2) =>
                    Ok(command::Command::Connection(
                        Box::new(command),
                        command::Connector::Pipe,
                        Box::new(command2)
                    )),
            }
    }    
}

fn build_tree_low(vector: Vec<String>) -> Result<command::Command, ()> {
    Ok(command::Command::SimpleCommand(command::SimpleCommand {
        cmd_line: vector,
        cmd_redirects: Vec::new(),
    }))
}