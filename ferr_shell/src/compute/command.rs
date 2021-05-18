use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub enum Connector {
    Seq, // &&
    And, // &
    Or, // ||
    Pipe, // |
}

pub enum Redirect {
    Input(String),
    Output(String),
    OutputAppend(String),
}

pub struct SimpleCommand {
    pub cmd_line: Vec<String>,
    pub cmd_redirects: Vec<Redirect>,
    pub cmd_bg: bool,
}

pub enum Command {
    Nothing,
    SimpleCommand(SimpleCommand),
    If(Box<Command>, Box<Command>, Box<Command>),
    Connection(Box<Command>, Connector, Box<Command>),
}


impl Command {
    pub fn make_async(self) -> Self {
        match self {
            Command::SimpleCommand(scmd) => {
                Command::SimpleCommand(SimpleCommand {
                    cmd_bg: true,
                    ..scmd
                })
            }
            Command::If(_, _, _) => panic!("not implemented"),
            Command::Connection(cmd1, c, cmd2) => {
                Command::Connection(cmd1, c, Box::new(cmd2.make_async()))
            }
            _ => self,
        }
    }
}
