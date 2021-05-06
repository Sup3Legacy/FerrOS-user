use alloc::string::String;

use super::command;

use alloc::boxed::Box;
use alloc::vec::Vec;

pub static mut FIRST_WORD: bool = true;

fn not_implemented(s: String) -> ! {
    let mut s2 = String::from("not implemented: ");
    s2.push_str(&s);
    panic!(&s2)
}

enum SimpleCommandElement {
    EltWord(String),
    EltRedi(command::Redirect),
}

enum my_unit {
    MyUnit,
}

fn make_simple_command(
    elt: SimpleCommandElement,
    mut cmd: command::SimpleCommand,
) -> command::SimpleCommand {
    match elt {
        SimpleCommandElement::EltWord(w) => {
            cmd.cmd_line.push(w);
            cmd
        }
        SimpleCommandElement::EltRedi(redi) => {
            cmd.cmd_redirects.push(redi);
            cmd
        }
    }
}

fn clean_simple_command(cmd: command::SimpleCommand) -> command::SimpleCommand {
    unsafe {
        FIRST_WORD = true;
    }
    cmd
}

pub enum Errors {
    ParsingError,
    LexingError(&'static str),
}

pub enum Token {
    NotAToken,
    IF,
    THEN,
    ELSE,
    ELIF,
    FI,
    LESS,
    GREATER,
    GREATER_GREATER,
    AND,
    AND_AND,
    NEWLINE,
    OR_OR,
    BAR,
    SEMI,
    EOF,
    WORD(String),
}
enum RulesType {
    COMMAND(command::Command),
    COMPOUND_LIST(command::Command),
    ELIF_CLAUSE(command::Command),
    IF_COMMAND(command::Command),
    INPUTUNIT(command::Command),
    LIST(command::Command),
    LIST0(command::Command),
    LIST1(command::Command),
    NEWLINE_LIST(my_unit),
    PIPELINE(command::Command),
    PIPELINE_COMMAND(command::Command),
    REDIRECTION(command::Redirect),
    SHELL_COMMAND(command::Command),
    SIMPLE_COMMAND(command::SimpleCommand),
    SIMPLE_COMMAND_ELEMENT(SimpleCommandElement),
    SIMPLE_LIST(command::Command),
    SIMPLE_LIST1(command::Command),
    SIMPLE_LIST_TERMINATOR(my_unit),
    Tok(Token),
}
enum ReturnReduce {
    SamenhirErrorReduce,
    Success(Vec<usize>, Vec<RulesType>, &'static str),
}

#[allow(dead_code)]
enum ActionTypes {
    Action(usize),
    Shift(usize),
    Success,
    Failure,
}

fn action_table(state: usize, next_token: &Token) -> ActionTypes {
    match (state, next_token) {
        (1, Token::GREATER) => ActionTypes::Shift(171),
        (1, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (1, Token::IF) => ActionTypes::Shift(58),
        (1, Token::LESS) => ActionTypes::Shift(187),
        (1, Token::NEWLINE) => ActionTypes::Shift(158),
        (1, Token::WORD(_)) => ActionTypes::Shift(207),
        (2, Token::ELIF) => ActionTypes::Action(0),
        (2, Token::ELSE) => ActionTypes::Action(0),
        (2, Token::FI) => ActionTypes::Action(0),
        (2, Token::GREATER) => ActionTypes::Shift(171),
        (2, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (2, Token::IF) => ActionTypes::Shift(58),
        (2, Token::LESS) => ActionTypes::Shift(187),
        (2, Token::NEWLINE) => ActionTypes::Shift(155),
        (2, Token::WORD(_)) => ActionTypes::Shift(207),
        (3, Token::ELIF) => ActionTypes::Action(1),
        (3, Token::ELSE) => ActionTypes::Action(1),
        (3, Token::FI) => ActionTypes::Action(1),
        (3, Token::GREATER) => ActionTypes::Shift(171),
        (3, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (3, Token::IF) => ActionTypes::Shift(58),
        (3, Token::LESS) => ActionTypes::Shift(187),
        (3, Token::NEWLINE) => ActionTypes::Shift(155),
        (3, Token::WORD(_)) => ActionTypes::Shift(207),
        (4, Token::ELIF) => ActionTypes::Action(2),
        (4, Token::ELSE) => ActionTypes::Action(2),
        (4, Token::FI) => ActionTypes::Action(2),
        (4, Token::GREATER) => ActionTypes::Shift(171),
        (4, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (4, Token::IF) => ActionTypes::Shift(58),
        (4, Token::LESS) => ActionTypes::Shift(187),
        (4, Token::NEWLINE) => ActionTypes::Shift(155),
        (4, Token::WORD(_)) => ActionTypes::Shift(207),
        (5, Token::GREATER) => ActionTypes::Shift(171),
        (5, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (5, Token::IF) => ActionTypes::Shift(58),
        (5, Token::LESS) => ActionTypes::Shift(187),
        (5, Token::NEWLINE) => ActionTypes::Shift(158),
        (5, Token::WORD(_)) => ActionTypes::Shift(207),
        (6, Token::GREATER) => ActionTypes::Shift(171),
        (6, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (6, Token::IF) => ActionTypes::Shift(58),
        (6, Token::LESS) => ActionTypes::Shift(187),
        (6, Token::NEWLINE) => ActionTypes::Shift(158),
        (6, Token::WORD(_)) => ActionTypes::Shift(207),
        (7, Token::GREATER) => ActionTypes::Shift(171),
        (7, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (7, Token::IF) => ActionTypes::Shift(58),
        (7, Token::LESS) => ActionTypes::Shift(187),
        (7, Token::NEWLINE) => ActionTypes::Shift(158),
        (7, Token::WORD(_)) => ActionTypes::Shift(207),
        (8, Token::GREATER) => ActionTypes::Shift(171),
        (8, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (8, Token::IF) => ActionTypes::Shift(58),
        (8, Token::LESS) => ActionTypes::Shift(187),
        (8, Token::NEWLINE) => ActionTypes::Shift(158),
        (8, Token::WORD(_)) => ActionTypes::Shift(207),
        (9, Token::GREATER) => ActionTypes::Shift(171),
        (9, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (9, Token::IF) => ActionTypes::Shift(58),
        (9, Token::LESS) => ActionTypes::Shift(187),
        (9, Token::NEWLINE) => ActionTypes::Shift(158),
        (9, Token::WORD(_)) => ActionTypes::Shift(207),
        (10, Token::GREATER) => ActionTypes::Shift(171),
        (10, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (10, Token::IF) => ActionTypes::Shift(58),
        (10, Token::LESS) => ActionTypes::Shift(187),
        (10, Token::NEWLINE) => ActionTypes::Shift(158),
        (10, Token::WORD(_)) => ActionTypes::Shift(207),
        (11, Token::EOF) => ActionTypes::Shift(100),
        (11, Token::GREATER) => ActionTypes::Shift(172),
        (11, Token::GREATER_GREATER) => ActionTypes::Shift(180),
        (11, Token::IF) => ActionTypes::Shift(59),
        (11, Token::LESS) => ActionTypes::Shift(188),
        (11, Token::NEWLINE) => ActionTypes::Shift(101),
        (11, Token::WORD(_)) => ActionTypes::Shift(208),
        (12, Token::GREATER) => ActionTypes::Shift(172),
        (12, Token::GREATER_GREATER) => ActionTypes::Shift(180),
        (12, Token::IF) => ActionTypes::Shift(59),
        (12, Token::LESS) => ActionTypes::Shift(188),
        (12, Token::NEWLINE) => ActionTypes::Shift(158),
        (12, Token::WORD(_)) => ActionTypes::Shift(208),
        (13, Token::GREATER) => ActionTypes::Shift(172),
        (13, Token::GREATER_GREATER) => ActionTypes::Shift(180),
        (13, Token::IF) => ActionTypes::Shift(59),
        (13, Token::LESS) => ActionTypes::Shift(188),
        (13, Token::NEWLINE) => ActionTypes::Shift(158),
        (13, Token::WORD(_)) => ActionTypes::Shift(208),
        (14, Token::GREATER) => ActionTypes::Shift(172),
        (14, Token::GREATER_GREATER) => ActionTypes::Shift(180),
        (14, Token::IF) => ActionTypes::Shift(59),
        (14, Token::LESS) => ActionTypes::Shift(188),
        (14, Token::NEWLINE) => ActionTypes::Shift(158),
        (14, Token::WORD(_)) => ActionTypes::Shift(208),
        (15, Token::EOF) => ActionTypes::Action(3),
        (15, Token::GREATER) => ActionTypes::Shift(172),
        (15, Token::GREATER_GREATER) => ActionTypes::Shift(180),
        (15, Token::IF) => ActionTypes::Shift(59),
        (15, Token::LESS) => ActionTypes::Shift(188),
        (15, Token::NEWLINE) => ActionTypes::Action(3),
        (15, Token::WORD(_)) => ActionTypes::Shift(208),
        (16, Token::EOF) => ActionTypes::Action(4),
        (16, Token::GREATER) => ActionTypes::Shift(172),
        (16, Token::GREATER_GREATER) => ActionTypes::Shift(180),
        (16, Token::IF) => ActionTypes::Shift(59),
        (16, Token::LESS) => ActionTypes::Shift(188),
        (16, Token::NEWLINE) => ActionTypes::Action(4),
        (16, Token::WORD(_)) => ActionTypes::Shift(208),
        (17, Token::GREATER) => ActionTypes::Shift(172),
        (17, Token::GREATER_GREATER) => ActionTypes::Shift(180),
        (17, Token::IF) => ActionTypes::Shift(59),
        (17, Token::LESS) => ActionTypes::Shift(188),
        (17, Token::WORD(_)) => ActionTypes::Shift(208),
        (18, Token::GREATER) => ActionTypes::Shift(172),
        (18, Token::GREATER_GREATER) => ActionTypes::Shift(180),
        (18, Token::IF) => ActionTypes::Shift(59),
        (18, Token::LESS) => ActionTypes::Shift(188),
        (18, Token::WORD(_)) => ActionTypes::Shift(208),
        (19, Token::GREATER) => ActionTypes::Shift(173),
        (19, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (19, Token::IF) => ActionTypes::Shift(60),
        (19, Token::LESS) => ActionTypes::Shift(189),
        (19, Token::NEWLINE) => ActionTypes::Shift(158),
        (19, Token::WORD(_)) => ActionTypes::Shift(209),
        (20, Token::FI) => ActionTypes::Action(0),
        (20, Token::GREATER) => ActionTypes::Shift(173),
        (20, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (20, Token::IF) => ActionTypes::Shift(60),
        (20, Token::LESS) => ActionTypes::Shift(189),
        (20, Token::NEWLINE) => ActionTypes::Shift(156),
        (20, Token::WORD(_)) => ActionTypes::Shift(209),
        (21, Token::FI) => ActionTypes::Action(1),
        (21, Token::GREATER) => ActionTypes::Shift(173),
        (21, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (21, Token::IF) => ActionTypes::Shift(60),
        (21, Token::LESS) => ActionTypes::Shift(189),
        (21, Token::NEWLINE) => ActionTypes::Shift(156),
        (21, Token::WORD(_)) => ActionTypes::Shift(209),
        (22, Token::FI) => ActionTypes::Action(2),
        (22, Token::GREATER) => ActionTypes::Shift(173),
        (22, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (22, Token::IF) => ActionTypes::Shift(60),
        (22, Token::LESS) => ActionTypes::Shift(189),
        (22, Token::NEWLINE) => ActionTypes::Shift(156),
        (22, Token::WORD(_)) => ActionTypes::Shift(209),
        (23, Token::GREATER) => ActionTypes::Shift(173),
        (23, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (23, Token::IF) => ActionTypes::Shift(60),
        (23, Token::LESS) => ActionTypes::Shift(189),
        (23, Token::NEWLINE) => ActionTypes::Shift(158),
        (23, Token::WORD(_)) => ActionTypes::Shift(209),
        (24, Token::GREATER) => ActionTypes::Shift(173),
        (24, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (24, Token::IF) => ActionTypes::Shift(60),
        (24, Token::LESS) => ActionTypes::Shift(189),
        (24, Token::NEWLINE) => ActionTypes::Shift(158),
        (24, Token::WORD(_)) => ActionTypes::Shift(209),
        (25, Token::GREATER) => ActionTypes::Shift(173),
        (25, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (25, Token::IF) => ActionTypes::Shift(60),
        (25, Token::LESS) => ActionTypes::Shift(189),
        (25, Token::NEWLINE) => ActionTypes::Shift(158),
        (25, Token::WORD(_)) => ActionTypes::Shift(209),
        (26, Token::GREATER) => ActionTypes::Shift(173),
        (26, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (26, Token::IF) => ActionTypes::Shift(60),
        (26, Token::LESS) => ActionTypes::Shift(189),
        (26, Token::NEWLINE) => ActionTypes::Shift(158),
        (26, Token::WORD(_)) => ActionTypes::Shift(209),
        (27, Token::GREATER) => ActionTypes::Shift(173),
        (27, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (27, Token::IF) => ActionTypes::Shift(60),
        (27, Token::LESS) => ActionTypes::Shift(189),
        (27, Token::NEWLINE) => ActionTypes::Shift(158),
        (27, Token::WORD(_)) => ActionTypes::Shift(209),
        (28, Token::GREATER) => ActionTypes::Shift(173),
        (28, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (28, Token::IF) => ActionTypes::Shift(60),
        (28, Token::LESS) => ActionTypes::Shift(189),
        (28, Token::NEWLINE) => ActionTypes::Shift(158),
        (28, Token::WORD(_)) => ActionTypes::Shift(209),
        (29, Token::GREATER) => ActionTypes::Shift(174),
        (29, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (29, Token::IF) => ActionTypes::Shift(61),
        (29, Token::LESS) => ActionTypes::Shift(190),
        (29, Token::NEWLINE) => ActionTypes::Shift(158),
        (29, Token::WORD(_)) => ActionTypes::Shift(210),
        (30, Token::GREATER) => ActionTypes::Shift(174),
        (30, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (30, Token::IF) => ActionTypes::Shift(61),
        (30, Token::LESS) => ActionTypes::Shift(190),
        (30, Token::NEWLINE) => ActionTypes::Shift(157),
        (30, Token::THEN) => ActionTypes::Action(0),
        (30, Token::WORD(_)) => ActionTypes::Shift(210),
        (31, Token::GREATER) => ActionTypes::Shift(174),
        (31, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (31, Token::IF) => ActionTypes::Shift(61),
        (31, Token::LESS) => ActionTypes::Shift(190),
        (31, Token::NEWLINE) => ActionTypes::Shift(157),
        (31, Token::THEN) => ActionTypes::Action(1),
        (31, Token::WORD(_)) => ActionTypes::Shift(210),
        (32, Token::GREATER) => ActionTypes::Shift(174),
        (32, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (32, Token::IF) => ActionTypes::Shift(61),
        (32, Token::LESS) => ActionTypes::Shift(190),
        (32, Token::NEWLINE) => ActionTypes::Shift(157),
        (32, Token::THEN) => ActionTypes::Action(2),
        (32, Token::WORD(_)) => ActionTypes::Shift(210),
        (33, Token::GREATER) => ActionTypes::Shift(174),
        (33, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (33, Token::IF) => ActionTypes::Shift(61),
        (33, Token::LESS) => ActionTypes::Shift(190),
        (33, Token::NEWLINE) => ActionTypes::Shift(158),
        (33, Token::WORD(_)) => ActionTypes::Shift(210),
        (34, Token::GREATER) => ActionTypes::Shift(174),
        (34, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (34, Token::IF) => ActionTypes::Shift(61),
        (34, Token::LESS) => ActionTypes::Shift(190),
        (34, Token::NEWLINE) => ActionTypes::Shift(158),
        (34, Token::WORD(_)) => ActionTypes::Shift(210),
        (35, Token::GREATER) => ActionTypes::Shift(174),
        (35, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (35, Token::IF) => ActionTypes::Shift(61),
        (35, Token::LESS) => ActionTypes::Shift(190),
        (35, Token::NEWLINE) => ActionTypes::Shift(158),
        (35, Token::WORD(_)) => ActionTypes::Shift(210),
        (36, Token::GREATER) => ActionTypes::Shift(174),
        (36, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (36, Token::IF) => ActionTypes::Shift(61),
        (36, Token::LESS) => ActionTypes::Shift(190),
        (36, Token::NEWLINE) => ActionTypes::Shift(158),
        (36, Token::WORD(_)) => ActionTypes::Shift(210),
        (37, Token::GREATER) => ActionTypes::Shift(174),
        (37, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (37, Token::IF) => ActionTypes::Shift(61),
        (37, Token::LESS) => ActionTypes::Shift(190),
        (37, Token::NEWLINE) => ActionTypes::Shift(158),
        (37, Token::WORD(_)) => ActionTypes::Shift(210),
        (38, Token::GREATER) => ActionTypes::Shift(174),
        (38, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (38, Token::IF) => ActionTypes::Shift(61),
        (38, Token::LESS) => ActionTypes::Shift(190),
        (38, Token::NEWLINE) => ActionTypes::Shift(158),
        (38, Token::WORD(_)) => ActionTypes::Shift(210),
        (39, Token::AND) => ActionTypes::Action(5),
        (39, Token::AND_AND) => ActionTypes::Action(5),
        (39, Token::BAR) => ActionTypes::Action(5),
        (39, Token::ELIF) => ActionTypes::Action(5),
        (39, Token::ELSE) => ActionTypes::Action(5),
        (39, Token::FI) => ActionTypes::Action(5),
        (39, Token::NEWLINE) => ActionTypes::Action(5),
        (39, Token::OR_OR) => ActionTypes::Action(5),
        (39, Token::SEMI) => ActionTypes::Action(5),
        (40, Token::AND) => ActionTypes::Action(5),
        (40, Token::AND_AND) => ActionTypes::Action(5),
        (40, Token::BAR) => ActionTypes::Action(5),
        (40, Token::EOF) => ActionTypes::Action(5),
        (40, Token::NEWLINE) => ActionTypes::Action(5),
        (40, Token::OR_OR) => ActionTypes::Action(5),
        (40, Token::SEMI) => ActionTypes::Action(5),
        (41, Token::AND) => ActionTypes::Action(5),
        (41, Token::AND_AND) => ActionTypes::Action(5),
        (41, Token::BAR) => ActionTypes::Action(5),
        (41, Token::FI) => ActionTypes::Action(5),
        (41, Token::NEWLINE) => ActionTypes::Action(5),
        (41, Token::OR_OR) => ActionTypes::Action(5),
        (41, Token::SEMI) => ActionTypes::Action(5),
        (42, Token::AND) => ActionTypes::Action(5),
        (42, Token::AND_AND) => ActionTypes::Action(5),
        (42, Token::BAR) => ActionTypes::Action(5),
        (42, Token::NEWLINE) => ActionTypes::Action(5),
        (42, Token::OR_OR) => ActionTypes::Action(5),
        (42, Token::SEMI) => ActionTypes::Action(5),
        (42, Token::THEN) => ActionTypes::Action(5),
        (43, Token::AND) => ActionTypes::Action(6),
        (43, Token::AND_AND) => ActionTypes::Action(6),
        (43, Token::BAR) => ActionTypes::Action(6),
        (43, Token::ELIF) => ActionTypes::Action(6),
        (43, Token::ELSE) => ActionTypes::Action(6),
        (43, Token::FI) => ActionTypes::Action(6),
        (43, Token::GREATER) => ActionTypes::Shift(171),
        (43, Token::GREATER_GREATER) => ActionTypes::Shift(179),
        (43, Token::LESS) => ActionTypes::Shift(187),
        (43, Token::NEWLINE) => ActionTypes::Action(6),
        (43, Token::OR_OR) => ActionTypes::Action(6),
        (43, Token::SEMI) => ActionTypes::Action(6),
        (43, Token::WORD(_)) => ActionTypes::Shift(207),
        (44, Token::AND) => ActionTypes::Action(6),
        (44, Token::AND_AND) => ActionTypes::Action(6),
        (44, Token::BAR) => ActionTypes::Action(6),
        (44, Token::EOF) => ActionTypes::Action(6),
        (44, Token::GREATER) => ActionTypes::Shift(172),
        (44, Token::GREATER_GREATER) => ActionTypes::Shift(180),
        (44, Token::LESS) => ActionTypes::Shift(188),
        (44, Token::NEWLINE) => ActionTypes::Action(6),
        (44, Token::OR_OR) => ActionTypes::Action(6),
        (44, Token::SEMI) => ActionTypes::Action(6),
        (44, Token::WORD(_)) => ActionTypes::Shift(208),
        (45, Token::AND) => ActionTypes::Action(6),
        (45, Token::AND_AND) => ActionTypes::Action(6),
        (45, Token::BAR) => ActionTypes::Action(6),
        (45, Token::FI) => ActionTypes::Action(6),
        (45, Token::GREATER) => ActionTypes::Shift(173),
        (45, Token::GREATER_GREATER) => ActionTypes::Shift(181),
        (45, Token::LESS) => ActionTypes::Shift(189),
        (45, Token::NEWLINE) => ActionTypes::Action(6),
        (45, Token::OR_OR) => ActionTypes::Action(6),
        (45, Token::SEMI) => ActionTypes::Action(6),
        (45, Token::WORD(_)) => ActionTypes::Shift(209),
        (46, Token::AND) => ActionTypes::Action(6),
        (46, Token::AND_AND) => ActionTypes::Action(6),
        (46, Token::BAR) => ActionTypes::Action(6),
        (46, Token::GREATER) => ActionTypes::Shift(174),
        (46, Token::GREATER_GREATER) => ActionTypes::Shift(182),
        (46, Token::LESS) => ActionTypes::Shift(190),
        (46, Token::NEWLINE) => ActionTypes::Action(6),
        (46, Token::OR_OR) => ActionTypes::Action(6),
        (46, Token::SEMI) => ActionTypes::Action(6),
        (46, Token::THEN) => ActionTypes::Action(6),
        (46, Token::WORD(_)) => ActionTypes::Shift(210),
        (47, Token::GREATER) => ActionTypes::Action(7),
        (47, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (47, Token::IF) => ActionTypes::Action(7),
        (47, Token::LESS) => ActionTypes::Action(7),
        (47, Token::NEWLINE) => ActionTypes::Action(7),
        (47, Token::WORD(_)) => ActionTypes::Action(7),
        (48, Token::GREATER) => ActionTypes::Action(7),
        (48, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (48, Token::IF) => ActionTypes::Action(7),
        (48, Token::LESS) => ActionTypes::Action(7),
        (48, Token::NEWLINE) => ActionTypes::Action(7),
        (48, Token::WORD(_)) => ActionTypes::Action(7),
        (49, Token::GREATER) => ActionTypes::Action(7),
        (49, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (49, Token::IF) => ActionTypes::Action(7),
        (49, Token::LESS) => ActionTypes::Action(7),
        (49, Token::NEWLINE) => ActionTypes::Action(7),
        (49, Token::WORD(_)) => ActionTypes::Action(7),
        (50, Token::GREATER) => ActionTypes::Action(7),
        (50, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (50, Token::IF) => ActionTypes::Action(7),
        (50, Token::LESS) => ActionTypes::Action(7),
        (50, Token::NEWLINE) => ActionTypes::Action(7),
        (50, Token::WORD(_)) => ActionTypes::Action(7),
        (51, Token::GREATER) => ActionTypes::Action(7),
        (51, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (51, Token::IF) => ActionTypes::Action(7),
        (51, Token::LESS) => ActionTypes::Action(7),
        (51, Token::NEWLINE) => ActionTypes::Action(7),
        (51, Token::WORD(_)) => ActionTypes::Action(7),
        (52, Token::GREATER) => ActionTypes::Action(7),
        (52, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (52, Token::IF) => ActionTypes::Action(7),
        (52, Token::LESS) => ActionTypes::Action(7),
        (52, Token::NEWLINE) => ActionTypes::Action(7),
        (52, Token::WORD(_)) => ActionTypes::Action(7),
        (53, Token::GREATER) => ActionTypes::Action(7),
        (53, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (53, Token::IF) => ActionTypes::Action(7),
        (53, Token::LESS) => ActionTypes::Action(7),
        (53, Token::NEWLINE) => ActionTypes::Action(7),
        (53, Token::WORD(_)) => ActionTypes::Action(7),
        (54, Token::GREATER) => ActionTypes::Action(7),
        (54, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (54, Token::IF) => ActionTypes::Action(7),
        (54, Token::LESS) => ActionTypes::Action(7),
        (54, Token::NEWLINE) => ActionTypes::Action(7),
        (54, Token::WORD(_)) => ActionTypes::Action(7),
        (55, Token::GREATER) => ActionTypes::Action(7),
        (55, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (55, Token::IF) => ActionTypes::Action(7),
        (55, Token::LESS) => ActionTypes::Action(7),
        (55, Token::NEWLINE) => ActionTypes::Action(7),
        (55, Token::WORD(_)) => ActionTypes::Action(7),
        (56, Token::GREATER) => ActionTypes::Action(7),
        (56, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (56, Token::IF) => ActionTypes::Action(7),
        (56, Token::LESS) => ActionTypes::Action(7),
        (56, Token::NEWLINE) => ActionTypes::Action(7),
        (56, Token::WORD(_)) => ActionTypes::Action(7),
        (57, Token::GREATER) => ActionTypes::Action(7),
        (57, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (57, Token::IF) => ActionTypes::Action(7),
        (57, Token::LESS) => ActionTypes::Action(7),
        (57, Token::NEWLINE) => ActionTypes::Action(7),
        (57, Token::WORD(_)) => ActionTypes::Action(7),
        (58, Token::GREATER) => ActionTypes::Action(7),
        (58, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (58, Token::IF) => ActionTypes::Action(7),
        (58, Token::LESS) => ActionTypes::Action(7),
        (58, Token::NEWLINE) => ActionTypes::Action(7),
        (58, Token::WORD(_)) => ActionTypes::Action(7),
        (59, Token::GREATER) => ActionTypes::Action(7),
        (59, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (59, Token::IF) => ActionTypes::Action(7),
        (59, Token::LESS) => ActionTypes::Action(7),
        (59, Token::NEWLINE) => ActionTypes::Action(7),
        (59, Token::WORD(_)) => ActionTypes::Action(7),
        (60, Token::GREATER) => ActionTypes::Action(7),
        (60, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (60, Token::IF) => ActionTypes::Action(7),
        (60, Token::LESS) => ActionTypes::Action(7),
        (60, Token::NEWLINE) => ActionTypes::Action(7),
        (60, Token::WORD(_)) => ActionTypes::Action(7),
        (61, Token::GREATER) => ActionTypes::Action(7),
        (61, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (61, Token::IF) => ActionTypes::Action(7),
        (61, Token::LESS) => ActionTypes::Action(7),
        (61, Token::NEWLINE) => ActionTypes::Action(7),
        (61, Token::WORD(_)) => ActionTypes::Action(7),
        (62, Token::ELIF) => ActionTypes::Action(8),
        (62, Token::ELSE) => ActionTypes::Action(8),
        (62, Token::FI) => ActionTypes::Action(8),
        (63, Token::FI) => ActionTypes::Action(8),
        (64, Token::THEN) => ActionTypes::Action(8),
        (65, Token::AND) => ActionTypes::Shift(107),
        (65, Token::AND_AND) => ActionTypes::Shift(134),
        (65, Token::ELIF) => ActionTypes::Action(9),
        (65, Token::ELSE) => ActionTypes::Action(9),
        (65, Token::FI) => ActionTypes::Action(9),
        (65, Token::NEWLINE) => ActionTypes::Shift(110),
        (65, Token::OR_OR) => ActionTypes::Shift(140),
        (65, Token::SEMI) => ActionTypes::Shift(113),
        (66, Token::AND) => ActionTypes::Shift(108),
        (66, Token::AND_AND) => ActionTypes::Shift(135),
        (66, Token::FI) => ActionTypes::Action(9),
        (66, Token::NEWLINE) => ActionTypes::Shift(111),
        (66, Token::OR_OR) => ActionTypes::Shift(141),
        (66, Token::SEMI) => ActionTypes::Shift(114),
        (67, Token::AND) => ActionTypes::Shift(109),
        (67, Token::AND_AND) => ActionTypes::Shift(136),
        (67, Token::NEWLINE) => ActionTypes::Shift(112),
        (67, Token::OR_OR) => ActionTypes::Shift(142),
        (67, Token::SEMI) => ActionTypes::Shift(115),
        (67, Token::THEN) => ActionTypes::Action(9),
        (68, Token::ELIF) => ActionTypes::Shift(57),
        (68, Token::ELSE) => ActionTypes::Shift(52),
        (68, Token::FI) => ActionTypes::Action(10),
        (69, Token::ELIF) => ActionTypes::Shift(57),
        (69, Token::ELSE) => ActionTypes::Shift(53),
        (69, Token::FI) => ActionTypes::Shift(88),
        (70, Token::ELIF) => ActionTypes::Shift(57),
        (70, Token::ELSE) => ActionTypes::Shift(54),
        (70, Token::FI) => ActionTypes::Shift(89),
        (71, Token::ELIF) => ActionTypes::Shift(57),
        (71, Token::ELSE) => ActionTypes::Shift(55),
        (71, Token::FI) => ActionTypes::Shift(90),
        (72, Token::ELIF) => ActionTypes::Shift(57),
        (72, Token::ELSE) => ActionTypes::Shift(56),
        (72, Token::FI) => ActionTypes::Shift(91),
        (73, Token::THEN) => ActionTypes::Shift(47),
        (74, Token::FI) => ActionTypes::Action(11),
        (75, Token::FI) => ActionTypes::Action(12),
        (76, Token::THEN) => ActionTypes::Shift(48),
        (77, Token::THEN) => ActionTypes::Shift(49),
        (78, Token::THEN) => ActionTypes::Shift(50),
        (79, Token::THEN) => ActionTypes::Shift(51),
        (80, Token::FI) => ActionTypes::Shift(84),
        (81, Token::FI) => ActionTypes::Shift(85),
        (82, Token::FI) => ActionTypes::Shift(86),
        (83, Token::FI) => ActionTypes::Shift(87),
        (84, Token::AND) => ActionTypes::Action(13),
        (84, Token::AND_AND) => ActionTypes::Action(13),
        (84, Token::BAR) => ActionTypes::Action(13),
        (84, Token::ELIF) => ActionTypes::Action(13),
        (84, Token::ELSE) => ActionTypes::Action(13),
        (84, Token::FI) => ActionTypes::Action(13),
        (84, Token::NEWLINE) => ActionTypes::Action(13),
        (84, Token::OR_OR) => ActionTypes::Action(13),
        (84, Token::SEMI) => ActionTypes::Action(13),
        (85, Token::AND) => ActionTypes::Action(13),
        (85, Token::AND_AND) => ActionTypes::Action(13),
        (85, Token::BAR) => ActionTypes::Action(13),
        (85, Token::EOF) => ActionTypes::Action(13),
        (85, Token::NEWLINE) => ActionTypes::Action(13),
        (85, Token::OR_OR) => ActionTypes::Action(13),
        (85, Token::SEMI) => ActionTypes::Action(13),
        (86, Token::AND) => ActionTypes::Action(13),
        (86, Token::AND_AND) => ActionTypes::Action(13),
        (86, Token::BAR) => ActionTypes::Action(13),
        (86, Token::FI) => ActionTypes::Action(13),
        (86, Token::NEWLINE) => ActionTypes::Action(13),
        (86, Token::OR_OR) => ActionTypes::Action(13),
        (86, Token::SEMI) => ActionTypes::Action(13),
        (87, Token::AND) => ActionTypes::Action(13),
        (87, Token::AND_AND) => ActionTypes::Action(13),
        (87, Token::BAR) => ActionTypes::Action(13),
        (87, Token::NEWLINE) => ActionTypes::Action(13),
        (87, Token::OR_OR) => ActionTypes::Action(13),
        (87, Token::SEMI) => ActionTypes::Action(13),
        (87, Token::THEN) => ActionTypes::Action(13),
        (88, Token::AND) => ActionTypes::Action(14),
        (88, Token::AND_AND) => ActionTypes::Action(14),
        (88, Token::BAR) => ActionTypes::Action(14),
        (88, Token::ELIF) => ActionTypes::Action(14),
        (88, Token::ELSE) => ActionTypes::Action(14),
        (88, Token::FI) => ActionTypes::Action(14),
        (88, Token::NEWLINE) => ActionTypes::Action(14),
        (88, Token::OR_OR) => ActionTypes::Action(14),
        (88, Token::SEMI) => ActionTypes::Action(14),
        (89, Token::AND) => ActionTypes::Action(14),
        (89, Token::AND_AND) => ActionTypes::Action(14),
        (89, Token::BAR) => ActionTypes::Action(14),
        (89, Token::EOF) => ActionTypes::Action(14),
        (89, Token::NEWLINE) => ActionTypes::Action(14),
        (89, Token::OR_OR) => ActionTypes::Action(14),
        (89, Token::SEMI) => ActionTypes::Action(14),
        (90, Token::AND) => ActionTypes::Action(14),
        (90, Token::AND_AND) => ActionTypes::Action(14),
        (90, Token::BAR) => ActionTypes::Action(14),
        (90, Token::FI) => ActionTypes::Action(14),
        (90, Token::NEWLINE) => ActionTypes::Action(14),
        (90, Token::OR_OR) => ActionTypes::Action(14),
        (90, Token::SEMI) => ActionTypes::Action(14),
        (91, Token::AND) => ActionTypes::Action(14),
        (91, Token::AND_AND) => ActionTypes::Action(14),
        (91, Token::BAR) => ActionTypes::Action(14),
        (91, Token::NEWLINE) => ActionTypes::Action(14),
        (91, Token::OR_OR) => ActionTypes::Action(14),
        (91, Token::SEMI) => ActionTypes::Action(14),
        (91, Token::THEN) => ActionTypes::Action(14),
        (92, Token::FI) => ActionTypes::Shift(96),
        (93, Token::FI) => ActionTypes::Shift(97),
        (94, Token::FI) => ActionTypes::Shift(98),
        (95, Token::FI) => ActionTypes::Shift(99),
        (96, Token::AND) => ActionTypes::Action(15),
        (96, Token::AND_AND) => ActionTypes::Action(15),
        (96, Token::BAR) => ActionTypes::Action(15),
        (96, Token::ELIF) => ActionTypes::Action(15),
        (96, Token::ELSE) => ActionTypes::Action(15),
        (96, Token::FI) => ActionTypes::Action(15),
        (96, Token::NEWLINE) => ActionTypes::Action(15),
        (96, Token::OR_OR) => ActionTypes::Action(15),
        (96, Token::SEMI) => ActionTypes::Action(15),
        (97, Token::AND) => ActionTypes::Action(15),
        (97, Token::AND_AND) => ActionTypes::Action(15),
        (97, Token::BAR) => ActionTypes::Action(15),
        (97, Token::EOF) => ActionTypes::Action(15),
        (97, Token::NEWLINE) => ActionTypes::Action(15),
        (97, Token::OR_OR) => ActionTypes::Action(15),
        (97, Token::SEMI) => ActionTypes::Action(15),
        (98, Token::AND) => ActionTypes::Action(15),
        (98, Token::AND_AND) => ActionTypes::Action(15),
        (98, Token::BAR) => ActionTypes::Action(15),
        (98, Token::FI) => ActionTypes::Action(15),
        (98, Token::NEWLINE) => ActionTypes::Action(15),
        (98, Token::OR_OR) => ActionTypes::Action(15),
        (98, Token::SEMI) => ActionTypes::Action(15),
        (99, Token::AND) => ActionTypes::Action(15),
        (99, Token::AND_AND) => ActionTypes::Action(15),
        (99, Token::BAR) => ActionTypes::Action(15),
        (99, Token::NEWLINE) => ActionTypes::Action(15),
        (99, Token::OR_OR) => ActionTypes::Action(15),
        (99, Token::SEMI) => ActionTypes::Action(15),
        (99, Token::THEN) => ActionTypes::Action(15),
        (100, Token::EOF) => ActionTypes::Action(16),
        (101, Token::EOF) => ActionTypes::Action(17),
        (102, Token::EOF) => ActionTypes::Shift(221),
        (102, Token::NEWLINE) => ActionTypes::Shift(222),
        (103, Token::EOF) => ActionTypes::Action(18),
        (104, Token::ELIF) => ActionTypes::Action(19),
        (104, Token::ELSE) => ActionTypes::Action(19),
        (104, Token::FI) => ActionTypes::Action(19),
        (105, Token::FI) => ActionTypes::Action(19),
        (106, Token::THEN) => ActionTypes::Action(19),
        (107, Token::ELIF) => ActionTypes::Action(7),
        (107, Token::ELSE) => ActionTypes::Action(7),
        (107, Token::FI) => ActionTypes::Action(7),
        (107, Token::GREATER) => ActionTypes::Action(7),
        (107, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (107, Token::IF) => ActionTypes::Action(7),
        (107, Token::LESS) => ActionTypes::Action(7),
        (107, Token::NEWLINE) => ActionTypes::Action(7),
        (107, Token::WORD(_)) => ActionTypes::Action(7),
        (108, Token::FI) => ActionTypes::Action(7),
        (108, Token::GREATER) => ActionTypes::Action(7),
        (108, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (108, Token::IF) => ActionTypes::Action(7),
        (108, Token::LESS) => ActionTypes::Action(7),
        (108, Token::NEWLINE) => ActionTypes::Action(7),
        (108, Token::WORD(_)) => ActionTypes::Action(7),
        (109, Token::GREATER) => ActionTypes::Action(7),
        (109, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (109, Token::IF) => ActionTypes::Action(7),
        (109, Token::LESS) => ActionTypes::Action(7),
        (109, Token::NEWLINE) => ActionTypes::Action(7),
        (109, Token::THEN) => ActionTypes::Action(7),
        (109, Token::WORD(_)) => ActionTypes::Action(7),
        (110, Token::ELIF) => ActionTypes::Action(7),
        (110, Token::ELSE) => ActionTypes::Action(7),
        (110, Token::FI) => ActionTypes::Action(7),
        (110, Token::GREATER) => ActionTypes::Action(7),
        (110, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (110, Token::IF) => ActionTypes::Action(7),
        (110, Token::LESS) => ActionTypes::Action(7),
        (110, Token::NEWLINE) => ActionTypes::Action(7),
        (110, Token::WORD(_)) => ActionTypes::Action(7),
        (111, Token::FI) => ActionTypes::Action(7),
        (111, Token::GREATER) => ActionTypes::Action(7),
        (111, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (111, Token::IF) => ActionTypes::Action(7),
        (111, Token::LESS) => ActionTypes::Action(7),
        (111, Token::NEWLINE) => ActionTypes::Action(7),
        (111, Token::WORD(_)) => ActionTypes::Action(7),
        (112, Token::GREATER) => ActionTypes::Action(7),
        (112, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (112, Token::IF) => ActionTypes::Action(7),
        (112, Token::LESS) => ActionTypes::Action(7),
        (112, Token::NEWLINE) => ActionTypes::Action(7),
        (112, Token::THEN) => ActionTypes::Action(7),
        (112, Token::WORD(_)) => ActionTypes::Action(7),
        (113, Token::ELIF) => ActionTypes::Action(7),
        (113, Token::ELSE) => ActionTypes::Action(7),
        (113, Token::FI) => ActionTypes::Action(7),
        (113, Token::GREATER) => ActionTypes::Action(7),
        (113, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (113, Token::IF) => ActionTypes::Action(7),
        (113, Token::LESS) => ActionTypes::Action(7),
        (113, Token::NEWLINE) => ActionTypes::Action(7),
        (113, Token::WORD(_)) => ActionTypes::Action(7),
        (114, Token::FI) => ActionTypes::Action(7),
        (114, Token::GREATER) => ActionTypes::Action(7),
        (114, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (114, Token::IF) => ActionTypes::Action(7),
        (114, Token::LESS) => ActionTypes::Action(7),
        (114, Token::NEWLINE) => ActionTypes::Action(7),
        (114, Token::WORD(_)) => ActionTypes::Action(7),
        (115, Token::GREATER) => ActionTypes::Action(7),
        (115, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (115, Token::IF) => ActionTypes::Action(7),
        (115, Token::LESS) => ActionTypes::Action(7),
        (115, Token::NEWLINE) => ActionTypes::Action(7),
        (115, Token::THEN) => ActionTypes::Action(7),
        (115, Token::WORD(_)) => ActionTypes::Action(7),
        (116, Token::AND) => ActionTypes::Action(20),
        (116, Token::AND_AND) => ActionTypes::Shift(134),
        (116, Token::ELIF) => ActionTypes::Action(20),
        (116, Token::ELSE) => ActionTypes::Action(20),
        (116, Token::FI) => ActionTypes::Action(20),
        (116, Token::NEWLINE) => ActionTypes::Shift(137),
        (116, Token::OR_OR) => ActionTypes::Shift(140),
        (116, Token::SEMI) => ActionTypes::Shift(143),
        (117, Token::AND) => ActionTypes::Action(21),
        (117, Token::AND_AND) => ActionTypes::Action(21),
        (117, Token::ELIF) => ActionTypes::Action(21),
        (117, Token::ELSE) => ActionTypes::Action(21),
        (117, Token::FI) => ActionTypes::Action(21),
        (117, Token::NEWLINE) => ActionTypes::Action(21),
        (117, Token::OR_OR) => ActionTypes::Action(21),
        (117, Token::SEMI) => ActionTypes::Action(21),
        (118, Token::AND) => ActionTypes::Action(22),
        (118, Token::AND_AND) => ActionTypes::Shift(134),
        (118, Token::ELIF) => ActionTypes::Action(22),
        (118, Token::ELSE) => ActionTypes::Action(22),
        (118, Token::FI) => ActionTypes::Action(22),
        (118, Token::NEWLINE) => ActionTypes::Action(22),
        (118, Token::OR_OR) => ActionTypes::Shift(140),
        (118, Token::SEMI) => ActionTypes::Action(22),
        (119, Token::AND) => ActionTypes::Action(23),
        (119, Token::AND_AND) => ActionTypes::Action(23),
        (119, Token::ELIF) => ActionTypes::Action(23),
        (119, Token::ELSE) => ActionTypes::Action(23),
        (119, Token::FI) => ActionTypes::Action(23),
        (119, Token::NEWLINE) => ActionTypes::Action(23),
        (119, Token::OR_OR) => ActionTypes::Action(23),
        (119, Token::SEMI) => ActionTypes::Action(23),
        (120, Token::AND) => ActionTypes::Action(24),
        (120, Token::AND_AND) => ActionTypes::Shift(134),
        (120, Token::ELIF) => ActionTypes::Action(24),
        (120, Token::ELSE) => ActionTypes::Action(24),
        (120, Token::FI) => ActionTypes::Action(24),
        (120, Token::NEWLINE) => ActionTypes::Action(24),
        (120, Token::OR_OR) => ActionTypes::Shift(140),
        (120, Token::SEMI) => ActionTypes::Action(24),
        (121, Token::AND) => ActionTypes::Action(20),
        (121, Token::AND_AND) => ActionTypes::Shift(135),
        (121, Token::FI) => ActionTypes::Action(20),
        (121, Token::NEWLINE) => ActionTypes::Shift(138),
        (121, Token::OR_OR) => ActionTypes::Shift(141),
        (121, Token::SEMI) => ActionTypes::Shift(144),
        (122, Token::AND) => ActionTypes::Action(21),
        (122, Token::AND_AND) => ActionTypes::Action(21),
        (122, Token::FI) => ActionTypes::Action(21),
        (122, Token::NEWLINE) => ActionTypes::Action(21),
        (122, Token::OR_OR) => ActionTypes::Action(21),
        (122, Token::SEMI) => ActionTypes::Action(21),
        (123, Token::AND) => ActionTypes::Action(22),
        (123, Token::AND_AND) => ActionTypes::Shift(135),
        (123, Token::FI) => ActionTypes::Action(22),
        (123, Token::NEWLINE) => ActionTypes::Action(22),
        (123, Token::OR_OR) => ActionTypes::Shift(141),
        (123, Token::SEMI) => ActionTypes::Action(22),
        (124, Token::AND) => ActionTypes::Action(23),
        (124, Token::AND_AND) => ActionTypes::Action(23),
        (124, Token::FI) => ActionTypes::Action(23),
        (124, Token::NEWLINE) => ActionTypes::Action(23),
        (124, Token::OR_OR) => ActionTypes::Action(23),
        (124, Token::SEMI) => ActionTypes::Action(23),
        (125, Token::AND) => ActionTypes::Action(24),
        (125, Token::AND_AND) => ActionTypes::Shift(135),
        (125, Token::FI) => ActionTypes::Action(24),
        (125, Token::NEWLINE) => ActionTypes::Action(24),
        (125, Token::OR_OR) => ActionTypes::Shift(141),
        (125, Token::SEMI) => ActionTypes::Action(24),
        (126, Token::AND) => ActionTypes::Action(20),
        (126, Token::AND_AND) => ActionTypes::Shift(136),
        (126, Token::NEWLINE) => ActionTypes::Shift(139),
        (126, Token::OR_OR) => ActionTypes::Shift(142),
        (126, Token::SEMI) => ActionTypes::Shift(145),
        (126, Token::THEN) => ActionTypes::Action(20),
        (127, Token::AND) => ActionTypes::Action(21),
        (127, Token::AND_AND) => ActionTypes::Action(21),
        (127, Token::NEWLINE) => ActionTypes::Action(21),
        (127, Token::OR_OR) => ActionTypes::Action(21),
        (127, Token::SEMI) => ActionTypes::Action(21),
        (127, Token::THEN) => ActionTypes::Action(21),
        (128, Token::AND) => ActionTypes::Action(22),
        (128, Token::AND_AND) => ActionTypes::Shift(136),
        (128, Token::NEWLINE) => ActionTypes::Action(22),
        (128, Token::OR_OR) => ActionTypes::Shift(142),
        (128, Token::SEMI) => ActionTypes::Action(22),
        (128, Token::THEN) => ActionTypes::Action(22),
        (129, Token::AND) => ActionTypes::Action(23),
        (129, Token::AND_AND) => ActionTypes::Action(23),
        (129, Token::NEWLINE) => ActionTypes::Action(23),
        (129, Token::OR_OR) => ActionTypes::Action(23),
        (129, Token::SEMI) => ActionTypes::Action(23),
        (129, Token::THEN) => ActionTypes::Action(23),
        (130, Token::AND) => ActionTypes::Action(24),
        (130, Token::AND_AND) => ActionTypes::Shift(136),
        (130, Token::NEWLINE) => ActionTypes::Action(24),
        (130, Token::OR_OR) => ActionTypes::Shift(142),
        (130, Token::SEMI) => ActionTypes::Action(24),
        (130, Token::THEN) => ActionTypes::Action(24),
        (131, Token::GREATER) => ActionTypes::Action(7),
        (131, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (131, Token::IF) => ActionTypes::Action(7),
        (131, Token::LESS) => ActionTypes::Action(7),
        (131, Token::NEWLINE) => ActionTypes::Action(7),
        (131, Token::WORD(_)) => ActionTypes::Action(7),
        (132, Token::GREATER) => ActionTypes::Action(7),
        (132, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (132, Token::IF) => ActionTypes::Action(7),
        (132, Token::LESS) => ActionTypes::Action(7),
        (132, Token::NEWLINE) => ActionTypes::Action(7),
        (132, Token::WORD(_)) => ActionTypes::Action(7),
        (133, Token::GREATER) => ActionTypes::Action(7),
        (133, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (133, Token::IF) => ActionTypes::Action(7),
        (133, Token::LESS) => ActionTypes::Action(7),
        (133, Token::NEWLINE) => ActionTypes::Action(7),
        (133, Token::WORD(_)) => ActionTypes::Action(7),
        (134, Token::GREATER) => ActionTypes::Action(7),
        (134, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (134, Token::IF) => ActionTypes::Action(7),
        (134, Token::LESS) => ActionTypes::Action(7),
        (134, Token::NEWLINE) => ActionTypes::Action(7),
        (134, Token::WORD(_)) => ActionTypes::Action(7),
        (135, Token::GREATER) => ActionTypes::Action(7),
        (135, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (135, Token::IF) => ActionTypes::Action(7),
        (135, Token::LESS) => ActionTypes::Action(7),
        (135, Token::NEWLINE) => ActionTypes::Action(7),
        (135, Token::WORD(_)) => ActionTypes::Action(7),
        (136, Token::GREATER) => ActionTypes::Action(7),
        (136, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (136, Token::IF) => ActionTypes::Action(7),
        (136, Token::LESS) => ActionTypes::Action(7),
        (136, Token::NEWLINE) => ActionTypes::Action(7),
        (136, Token::WORD(_)) => ActionTypes::Action(7),
        (137, Token::GREATER) => ActionTypes::Action(7),
        (137, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (137, Token::IF) => ActionTypes::Action(7),
        (137, Token::LESS) => ActionTypes::Action(7),
        (137, Token::NEWLINE) => ActionTypes::Action(7),
        (137, Token::WORD(_)) => ActionTypes::Action(7),
        (138, Token::GREATER) => ActionTypes::Action(7),
        (138, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (138, Token::IF) => ActionTypes::Action(7),
        (138, Token::LESS) => ActionTypes::Action(7),
        (138, Token::NEWLINE) => ActionTypes::Action(7),
        (138, Token::WORD(_)) => ActionTypes::Action(7),
        (139, Token::GREATER) => ActionTypes::Action(7),
        (139, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (139, Token::IF) => ActionTypes::Action(7),
        (139, Token::LESS) => ActionTypes::Action(7),
        (139, Token::NEWLINE) => ActionTypes::Action(7),
        (139, Token::WORD(_)) => ActionTypes::Action(7),
        (140, Token::GREATER) => ActionTypes::Action(7),
        (140, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (140, Token::IF) => ActionTypes::Action(7),
        (140, Token::LESS) => ActionTypes::Action(7),
        (140, Token::NEWLINE) => ActionTypes::Action(7),
        (140, Token::WORD(_)) => ActionTypes::Action(7),
        (141, Token::GREATER) => ActionTypes::Action(7),
        (141, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (141, Token::IF) => ActionTypes::Action(7),
        (141, Token::LESS) => ActionTypes::Action(7),
        (141, Token::NEWLINE) => ActionTypes::Action(7),
        (141, Token::WORD(_)) => ActionTypes::Action(7),
        (142, Token::GREATER) => ActionTypes::Action(7),
        (142, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (142, Token::IF) => ActionTypes::Action(7),
        (142, Token::LESS) => ActionTypes::Action(7),
        (142, Token::NEWLINE) => ActionTypes::Action(7),
        (142, Token::WORD(_)) => ActionTypes::Action(7),
        (143, Token::GREATER) => ActionTypes::Action(7),
        (143, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (143, Token::IF) => ActionTypes::Action(7),
        (143, Token::LESS) => ActionTypes::Action(7),
        (143, Token::NEWLINE) => ActionTypes::Action(7),
        (143, Token::WORD(_)) => ActionTypes::Action(7),
        (144, Token::GREATER) => ActionTypes::Action(7),
        (144, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (144, Token::IF) => ActionTypes::Action(7),
        (144, Token::LESS) => ActionTypes::Action(7),
        (144, Token::NEWLINE) => ActionTypes::Action(7),
        (144, Token::WORD(_)) => ActionTypes::Action(7),
        (145, Token::GREATER) => ActionTypes::Action(7),
        (145, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (145, Token::IF) => ActionTypes::Action(7),
        (145, Token::LESS) => ActionTypes::Action(7),
        (145, Token::NEWLINE) => ActionTypes::Action(7),
        (145, Token::WORD(_)) => ActionTypes::Action(7),
        (146, Token::AND) => ActionTypes::Action(25),
        (146, Token::AND_AND) => ActionTypes::Action(25),
        (146, Token::ELIF) => ActionTypes::Action(25),
        (146, Token::ELSE) => ActionTypes::Action(25),
        (146, Token::FI) => ActionTypes::Action(25),
        (146, Token::NEWLINE) => ActionTypes::Action(25),
        (146, Token::OR_OR) => ActionTypes::Action(25),
        (146, Token::SEMI) => ActionTypes::Action(25),
        (147, Token::AND) => ActionTypes::Action(25),
        (147, Token::AND_AND) => ActionTypes::Action(25),
        (147, Token::FI) => ActionTypes::Action(25),
        (147, Token::NEWLINE) => ActionTypes::Action(25),
        (147, Token::OR_OR) => ActionTypes::Action(25),
        (147, Token::SEMI) => ActionTypes::Action(25),
        (148, Token::AND) => ActionTypes::Action(25),
        (148, Token::AND_AND) => ActionTypes::Action(25),
        (148, Token::NEWLINE) => ActionTypes::Action(25),
        (148, Token::OR_OR) => ActionTypes::Action(25),
        (148, Token::SEMI) => ActionTypes::Action(25),
        (148, Token::THEN) => ActionTypes::Action(25),
        (149, Token::GREATER) => ActionTypes::Action(7),
        (149, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (149, Token::IF) => ActionTypes::Action(7),
        (149, Token::LESS) => ActionTypes::Action(7),
        (149, Token::NEWLINE) => ActionTypes::Action(7),
        (149, Token::WORD(_)) => ActionTypes::Action(7),
        (150, Token::GREATER) => ActionTypes::Action(7),
        (150, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (150, Token::IF) => ActionTypes::Action(7),
        (150, Token::LESS) => ActionTypes::Action(7),
        (150, Token::NEWLINE) => ActionTypes::Action(7),
        (150, Token::WORD(_)) => ActionTypes::Action(7),
        (151, Token::GREATER) => ActionTypes::Action(7),
        (151, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (151, Token::IF) => ActionTypes::Action(7),
        (151, Token::LESS) => ActionTypes::Action(7),
        (151, Token::NEWLINE) => ActionTypes::Action(7),
        (151, Token::WORD(_)) => ActionTypes::Action(7),
        (152, Token::GREATER) => ActionTypes::Action(7),
        (152, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (152, Token::IF) => ActionTypes::Action(7),
        (152, Token::LESS) => ActionTypes::Action(7),
        (152, Token::NEWLINE) => ActionTypes::Action(7),
        (152, Token::WORD(_)) => ActionTypes::Action(7),
        (153, Token::GREATER) => ActionTypes::Action(7),
        (153, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (153, Token::IF) => ActionTypes::Action(7),
        (153, Token::LESS) => ActionTypes::Action(7),
        (153, Token::NEWLINE) => ActionTypes::Action(7),
        (153, Token::WORD(_)) => ActionTypes::Action(7),
        (154, Token::GREATER) => ActionTypes::Action(7),
        (154, Token::GREATER_GREATER) => ActionTypes::Action(7),
        (154, Token::IF) => ActionTypes::Action(7),
        (154, Token::LESS) => ActionTypes::Action(7),
        (154, Token::NEWLINE) => ActionTypes::Action(7),
        (154, Token::WORD(_)) => ActionTypes::Action(7),
        (155, Token::ELIF) => ActionTypes::Action(26),
        (155, Token::ELSE) => ActionTypes::Action(26),
        (155, Token::FI) => ActionTypes::Action(26),
        (155, Token::GREATER) => ActionTypes::Action(26),
        (155, Token::GREATER_GREATER) => ActionTypes::Action(26),
        (155, Token::IF) => ActionTypes::Action(26),
        (155, Token::LESS) => ActionTypes::Action(26),
        (155, Token::NEWLINE) => ActionTypes::Action(26),
        (155, Token::WORD(_)) => ActionTypes::Action(26),
        (156, Token::FI) => ActionTypes::Action(26),
        (156, Token::GREATER) => ActionTypes::Action(26),
        (156, Token::GREATER_GREATER) => ActionTypes::Action(26),
        (156, Token::IF) => ActionTypes::Action(26),
        (156, Token::LESS) => ActionTypes::Action(26),
        (156, Token::NEWLINE) => ActionTypes::Action(26),
        (156, Token::WORD(_)) => ActionTypes::Action(26),
        (157, Token::GREATER) => ActionTypes::Action(26),
        (157, Token::GREATER_GREATER) => ActionTypes::Action(26),
        (157, Token::IF) => ActionTypes::Action(26),
        (157, Token::LESS) => ActionTypes::Action(26),
        (157, Token::NEWLINE) => ActionTypes::Action(26),
        (157, Token::THEN) => ActionTypes::Action(26),
        (157, Token::WORD(_)) => ActionTypes::Action(26),
        (158, Token::GREATER) => ActionTypes::Action(26),
        (158, Token::GREATER_GREATER) => ActionTypes::Action(26),
        (158, Token::IF) => ActionTypes::Action(26),
        (158, Token::LESS) => ActionTypes::Action(26),
        (158, Token::NEWLINE) => ActionTypes::Action(26),
        (158, Token::WORD(_)) => ActionTypes::Action(26),
        (159, Token::AND) => ActionTypes::Action(27),
        (159, Token::AND_AND) => ActionTypes::Action(27),
        (159, Token::BAR) => ActionTypes::Action(27),
        (159, Token::ELIF) => ActionTypes::Action(27),
        (159, Token::ELSE) => ActionTypes::Action(27),
        (159, Token::FI) => ActionTypes::Action(27),
        (159, Token::NEWLINE) => ActionTypes::Action(27),
        (159, Token::OR_OR) => ActionTypes::Action(27),
        (159, Token::SEMI) => ActionTypes::Action(27),
        (160, Token::AND) => ActionTypes::Action(27),
        (160, Token::AND_AND) => ActionTypes::Action(27),
        (160, Token::BAR) => ActionTypes::Action(27),
        (160, Token::EOF) => ActionTypes::Action(27),
        (160, Token::NEWLINE) => ActionTypes::Action(27),
        (160, Token::OR_OR) => ActionTypes::Action(27),
        (160, Token::SEMI) => ActionTypes::Action(27),
        (161, Token::AND) => ActionTypes::Action(27),
        (161, Token::AND_AND) => ActionTypes::Action(27),
        (161, Token::BAR) => ActionTypes::Action(27),
        (161, Token::FI) => ActionTypes::Action(27),
        (161, Token::NEWLINE) => ActionTypes::Action(27),
        (161, Token::OR_OR) => ActionTypes::Action(27),
        (161, Token::SEMI) => ActionTypes::Action(27),
        (162, Token::AND) => ActionTypes::Action(27),
        (162, Token::AND_AND) => ActionTypes::Action(27),
        (162, Token::BAR) => ActionTypes::Action(27),
        (162, Token::NEWLINE) => ActionTypes::Action(27),
        (162, Token::OR_OR) => ActionTypes::Action(27),
        (162, Token::SEMI) => ActionTypes::Action(27),
        (162, Token::THEN) => ActionTypes::Action(27),
        (163, Token::AND) => ActionTypes::Action(28),
        (163, Token::AND_AND) => ActionTypes::Action(28),
        (163, Token::BAR) => ActionTypes::Shift(149),
        (163, Token::ELIF) => ActionTypes::Action(28),
        (163, Token::ELSE) => ActionTypes::Action(28),
        (163, Token::FI) => ActionTypes::Action(28),
        (163, Token::NEWLINE) => ActionTypes::Action(28),
        (163, Token::OR_OR) => ActionTypes::Action(28),
        (163, Token::SEMI) => ActionTypes::Action(28),
        (164, Token::AND) => ActionTypes::Action(29),
        (164, Token::AND_AND) => ActionTypes::Action(29),
        (164, Token::BAR) => ActionTypes::Shift(149),
        (164, Token::ELIF) => ActionTypes::Action(29),
        (164, Token::ELSE) => ActionTypes::Action(29),
        (164, Token::FI) => ActionTypes::Action(29),
        (164, Token::NEWLINE) => ActionTypes::Action(29),
        (164, Token::OR_OR) => ActionTypes::Action(29),
        (164, Token::SEMI) => ActionTypes::Action(29),
        (165, Token::AND) => ActionTypes::Action(28),
        (165, Token::AND_AND) => ActionTypes::Action(28),
        (165, Token::BAR) => ActionTypes::Shift(150),
        (165, Token::EOF) => ActionTypes::Action(28),
        (165, Token::NEWLINE) => ActionTypes::Action(28),
        (165, Token::OR_OR) => ActionTypes::Action(28),
        (165, Token::SEMI) => ActionTypes::Action(28),
        (166, Token::AND) => ActionTypes::Action(29),
        (166, Token::AND_AND) => ActionTypes::Action(29),
        (166, Token::BAR) => ActionTypes::Shift(150),
        (166, Token::EOF) => ActionTypes::Action(29),
        (166, Token::NEWLINE) => ActionTypes::Action(29),
        (166, Token::OR_OR) => ActionTypes::Action(29),
        (166, Token::SEMI) => ActionTypes::Action(29),
        (167, Token::AND) => ActionTypes::Action(28),
        (167, Token::AND_AND) => ActionTypes::Action(28),
        (167, Token::BAR) => ActionTypes::Shift(151),
        (167, Token::FI) => ActionTypes::Action(28),
        (167, Token::NEWLINE) => ActionTypes::Action(28),
        (167, Token::OR_OR) => ActionTypes::Action(28),
        (167, Token::SEMI) => ActionTypes::Action(28),
        (168, Token::AND) => ActionTypes::Action(29),
        (168, Token::AND_AND) => ActionTypes::Action(29),
        (168, Token::BAR) => ActionTypes::Shift(151),
        (168, Token::FI) => ActionTypes::Action(29),
        (168, Token::NEWLINE) => ActionTypes::Action(29),
        (168, Token::OR_OR) => ActionTypes::Action(29),
        (168, Token::SEMI) => ActionTypes::Action(29),
        (169, Token::AND) => ActionTypes::Action(28),
        (169, Token::AND_AND) => ActionTypes::Action(28),
        (169, Token::BAR) => ActionTypes::Shift(152),
        (169, Token::NEWLINE) => ActionTypes::Action(28),
        (169, Token::OR_OR) => ActionTypes::Action(28),
        (169, Token::SEMI) => ActionTypes::Action(28),
        (169, Token::THEN) => ActionTypes::Action(28),
        (170, Token::AND) => ActionTypes::Action(29),
        (170, Token::AND_AND) => ActionTypes::Action(29),
        (170, Token::BAR) => ActionTypes::Shift(152),
        (170, Token::NEWLINE) => ActionTypes::Action(29),
        (170, Token::OR_OR) => ActionTypes::Action(29),
        (170, Token::SEMI) => ActionTypes::Action(29),
        (170, Token::THEN) => ActionTypes::Action(29),
        (171, Token::WORD(_)) => ActionTypes::Shift(175),
        (172, Token::WORD(_)) => ActionTypes::Shift(176),
        (173, Token::WORD(_)) => ActionTypes::Shift(177),
        (174, Token::WORD(_)) => ActionTypes::Shift(178),
        (175, Token::AND) => ActionTypes::Action(30),
        (175, Token::AND_AND) => ActionTypes::Action(30),
        (175, Token::BAR) => ActionTypes::Action(30),
        (175, Token::ELIF) => ActionTypes::Action(30),
        (175, Token::ELSE) => ActionTypes::Action(30),
        (175, Token::FI) => ActionTypes::Action(30),
        (175, Token::GREATER) => ActionTypes::Action(30),
        (175, Token::GREATER_GREATER) => ActionTypes::Action(30),
        (175, Token::LESS) => ActionTypes::Action(30),
        (175, Token::NEWLINE) => ActionTypes::Action(30),
        (175, Token::OR_OR) => ActionTypes::Action(30),
        (175, Token::SEMI) => ActionTypes::Action(30),
        (175, Token::WORD(_)) => ActionTypes::Action(30),
        (176, Token::AND) => ActionTypes::Action(30),
        (176, Token::AND_AND) => ActionTypes::Action(30),
        (176, Token::BAR) => ActionTypes::Action(30),
        (176, Token::EOF) => ActionTypes::Action(30),
        (176, Token::GREATER) => ActionTypes::Action(30),
        (176, Token::GREATER_GREATER) => ActionTypes::Action(30),
        (176, Token::LESS) => ActionTypes::Action(30),
        (176, Token::NEWLINE) => ActionTypes::Action(30),
        (176, Token::OR_OR) => ActionTypes::Action(30),
        (176, Token::SEMI) => ActionTypes::Action(30),
        (176, Token::WORD(_)) => ActionTypes::Action(30),
        (177, Token::AND) => ActionTypes::Action(30),
        (177, Token::AND_AND) => ActionTypes::Action(30),
        (177, Token::BAR) => ActionTypes::Action(30),
        (177, Token::FI) => ActionTypes::Action(30),
        (177, Token::GREATER) => ActionTypes::Action(30),
        (177, Token::GREATER_GREATER) => ActionTypes::Action(30),
        (177, Token::LESS) => ActionTypes::Action(30),
        (177, Token::NEWLINE) => ActionTypes::Action(30),
        (177, Token::OR_OR) => ActionTypes::Action(30),
        (177, Token::SEMI) => ActionTypes::Action(30),
        (177, Token::WORD(_)) => ActionTypes::Action(30),
        (178, Token::AND) => ActionTypes::Action(30),
        (178, Token::AND_AND) => ActionTypes::Action(30),
        (178, Token::BAR) => ActionTypes::Action(30),
        (178, Token::GREATER) => ActionTypes::Action(30),
        (178, Token::GREATER_GREATER) => ActionTypes::Action(30),
        (178, Token::LESS) => ActionTypes::Action(30),
        (178, Token::NEWLINE) => ActionTypes::Action(30),
        (178, Token::OR_OR) => ActionTypes::Action(30),
        (178, Token::SEMI) => ActionTypes::Action(30),
        (178, Token::THEN) => ActionTypes::Action(30),
        (178, Token::WORD(_)) => ActionTypes::Action(30),
        (179, Token::WORD(_)) => ActionTypes::Shift(183),
        (180, Token::WORD(_)) => ActionTypes::Shift(184),
        (181, Token::WORD(_)) => ActionTypes::Shift(185),
        (182, Token::WORD(_)) => ActionTypes::Shift(186),
        (183, Token::AND) => ActionTypes::Action(31),
        (183, Token::AND_AND) => ActionTypes::Action(31),
        (183, Token::BAR) => ActionTypes::Action(31),
        (183, Token::ELIF) => ActionTypes::Action(31),
        (183, Token::ELSE) => ActionTypes::Action(31),
        (183, Token::FI) => ActionTypes::Action(31),
        (183, Token::GREATER) => ActionTypes::Action(31),
        (183, Token::GREATER_GREATER) => ActionTypes::Action(31),
        (183, Token::LESS) => ActionTypes::Action(31),
        (183, Token::NEWLINE) => ActionTypes::Action(31),
        (183, Token::OR_OR) => ActionTypes::Action(31),
        (183, Token::SEMI) => ActionTypes::Action(31),
        (183, Token::WORD(_)) => ActionTypes::Action(31),
        (184, Token::AND) => ActionTypes::Action(31),
        (184, Token::AND_AND) => ActionTypes::Action(31),
        (184, Token::BAR) => ActionTypes::Action(31),
        (184, Token::EOF) => ActionTypes::Action(31),
        (184, Token::GREATER) => ActionTypes::Action(31),
        (184, Token::GREATER_GREATER) => ActionTypes::Action(31),
        (184, Token::LESS) => ActionTypes::Action(31),
        (184, Token::NEWLINE) => ActionTypes::Action(31),
        (184, Token::OR_OR) => ActionTypes::Action(31),
        (184, Token::SEMI) => ActionTypes::Action(31),
        (184, Token::WORD(_)) => ActionTypes::Action(31),
        (185, Token::AND) => ActionTypes::Action(31),
        (185, Token::AND_AND) => ActionTypes::Action(31),
        (185, Token::BAR) => ActionTypes::Action(31),
        (185, Token::FI) => ActionTypes::Action(31),
        (185, Token::GREATER) => ActionTypes::Action(31),
        (185, Token::GREATER_GREATER) => ActionTypes::Action(31),
        (185, Token::LESS) => ActionTypes::Action(31),
        (185, Token::NEWLINE) => ActionTypes::Action(31),
        (185, Token::OR_OR) => ActionTypes::Action(31),
        (185, Token::SEMI) => ActionTypes::Action(31),
        (185, Token::WORD(_)) => ActionTypes::Action(31),
        (186, Token::AND) => ActionTypes::Action(31),
        (186, Token::AND_AND) => ActionTypes::Action(31),
        (186, Token::BAR) => ActionTypes::Action(31),
        (186, Token::GREATER) => ActionTypes::Action(31),
        (186, Token::GREATER_GREATER) => ActionTypes::Action(31),
        (186, Token::LESS) => ActionTypes::Action(31),
        (186, Token::NEWLINE) => ActionTypes::Action(31),
        (186, Token::OR_OR) => ActionTypes::Action(31),
        (186, Token::SEMI) => ActionTypes::Action(31),
        (186, Token::THEN) => ActionTypes::Action(31),
        (186, Token::WORD(_)) => ActionTypes::Action(31),
        (187, Token::WORD(_)) => ActionTypes::Shift(191),
        (188, Token::WORD(_)) => ActionTypes::Shift(192),
        (189, Token::WORD(_)) => ActionTypes::Shift(193),
        (190, Token::WORD(_)) => ActionTypes::Shift(194),
        (191, Token::AND) => ActionTypes::Action(32),
        (191, Token::AND_AND) => ActionTypes::Action(32),
        (191, Token::BAR) => ActionTypes::Action(32),
        (191, Token::ELIF) => ActionTypes::Action(32),
        (191, Token::ELSE) => ActionTypes::Action(32),
        (191, Token::FI) => ActionTypes::Action(32),
        (191, Token::GREATER) => ActionTypes::Action(32),
        (191, Token::GREATER_GREATER) => ActionTypes::Action(32),
        (191, Token::LESS) => ActionTypes::Action(32),
        (191, Token::NEWLINE) => ActionTypes::Action(32),
        (191, Token::OR_OR) => ActionTypes::Action(32),
        (191, Token::SEMI) => ActionTypes::Action(32),
        (191, Token::WORD(_)) => ActionTypes::Action(32),
        (192, Token::AND) => ActionTypes::Action(32),
        (192, Token::AND_AND) => ActionTypes::Action(32),
        (192, Token::BAR) => ActionTypes::Action(32),
        (192, Token::EOF) => ActionTypes::Action(32),
        (192, Token::GREATER) => ActionTypes::Action(32),
        (192, Token::GREATER_GREATER) => ActionTypes::Action(32),
        (192, Token::LESS) => ActionTypes::Action(32),
        (192, Token::NEWLINE) => ActionTypes::Action(32),
        (192, Token::OR_OR) => ActionTypes::Action(32),
        (192, Token::SEMI) => ActionTypes::Action(32),
        (192, Token::WORD(_)) => ActionTypes::Action(32),
        (193, Token::AND) => ActionTypes::Action(32),
        (193, Token::AND_AND) => ActionTypes::Action(32),
        (193, Token::BAR) => ActionTypes::Action(32),
        (193, Token::FI) => ActionTypes::Action(32),
        (193, Token::GREATER) => ActionTypes::Action(32),
        (193, Token::GREATER_GREATER) => ActionTypes::Action(32),
        (193, Token::LESS) => ActionTypes::Action(32),
        (193, Token::NEWLINE) => ActionTypes::Action(32),
        (193, Token::OR_OR) => ActionTypes::Action(32),
        (193, Token::SEMI) => ActionTypes::Action(32),
        (193, Token::WORD(_)) => ActionTypes::Action(32),
        (194, Token::AND) => ActionTypes::Action(32),
        (194, Token::AND_AND) => ActionTypes::Action(32),
        (194, Token::BAR) => ActionTypes::Action(32),
        (194, Token::GREATER) => ActionTypes::Action(32),
        (194, Token::GREATER_GREATER) => ActionTypes::Action(32),
        (194, Token::LESS) => ActionTypes::Action(32),
        (194, Token::NEWLINE) => ActionTypes::Action(32),
        (194, Token::OR_OR) => ActionTypes::Action(32),
        (194, Token::SEMI) => ActionTypes::Action(32),
        (194, Token::THEN) => ActionTypes::Action(32),
        (194, Token::WORD(_)) => ActionTypes::Action(32),
        (195, Token::AND) => ActionTypes::Action(33),
        (195, Token::AND_AND) => ActionTypes::Action(33),
        (195, Token::BAR) => ActionTypes::Action(33),
        (195, Token::ELIF) => ActionTypes::Action(33),
        (195, Token::ELSE) => ActionTypes::Action(33),
        (195, Token::FI) => ActionTypes::Action(33),
        (195, Token::NEWLINE) => ActionTypes::Action(33),
        (195, Token::OR_OR) => ActionTypes::Action(33),
        (195, Token::SEMI) => ActionTypes::Action(33),
        (196, Token::AND) => ActionTypes::Action(33),
        (196, Token::AND_AND) => ActionTypes::Action(33),
        (196, Token::BAR) => ActionTypes::Action(33),
        (196, Token::EOF) => ActionTypes::Action(33),
        (196, Token::NEWLINE) => ActionTypes::Action(33),
        (196, Token::OR_OR) => ActionTypes::Action(33),
        (196, Token::SEMI) => ActionTypes::Action(33),
        (197, Token::AND) => ActionTypes::Action(33),
        (197, Token::AND_AND) => ActionTypes::Action(33),
        (197, Token::BAR) => ActionTypes::Action(33),
        (197, Token::FI) => ActionTypes::Action(33),
        (197, Token::NEWLINE) => ActionTypes::Action(33),
        (197, Token::OR_OR) => ActionTypes::Action(33),
        (197, Token::SEMI) => ActionTypes::Action(33),
        (198, Token::AND) => ActionTypes::Action(33),
        (198, Token::AND_AND) => ActionTypes::Action(33),
        (198, Token::BAR) => ActionTypes::Action(33),
        (198, Token::NEWLINE) => ActionTypes::Action(33),
        (198, Token::OR_OR) => ActionTypes::Action(33),
        (198, Token::SEMI) => ActionTypes::Action(33),
        (198, Token::THEN) => ActionTypes::Action(33),
        (199, Token::AND) => ActionTypes::Action(34),
        (199, Token::AND_AND) => ActionTypes::Action(34),
        (199, Token::BAR) => ActionTypes::Action(34),
        (199, Token::ELIF) => ActionTypes::Action(34),
        (199, Token::ELSE) => ActionTypes::Action(34),
        (199, Token::FI) => ActionTypes::Action(34),
        (199, Token::GREATER) => ActionTypes::Action(34),
        (199, Token::GREATER_GREATER) => ActionTypes::Action(34),
        (199, Token::LESS) => ActionTypes::Action(34),
        (199, Token::NEWLINE) => ActionTypes::Action(34),
        (199, Token::OR_OR) => ActionTypes::Action(34),
        (199, Token::SEMI) => ActionTypes::Action(34),
        (199, Token::WORD(_)) => ActionTypes::Action(34),
        (200, Token::AND) => ActionTypes::Action(34),
        (200, Token::AND_AND) => ActionTypes::Action(34),
        (200, Token::BAR) => ActionTypes::Action(34),
        (200, Token::EOF) => ActionTypes::Action(34),
        (200, Token::GREATER) => ActionTypes::Action(34),
        (200, Token::GREATER_GREATER) => ActionTypes::Action(34),
        (200, Token::LESS) => ActionTypes::Action(34),
        (200, Token::NEWLINE) => ActionTypes::Action(34),
        (200, Token::OR_OR) => ActionTypes::Action(34),
        (200, Token::SEMI) => ActionTypes::Action(34),
        (200, Token::WORD(_)) => ActionTypes::Action(34),
        (201, Token::AND) => ActionTypes::Action(34),
        (201, Token::AND_AND) => ActionTypes::Action(34),
        (201, Token::BAR) => ActionTypes::Action(34),
        (201, Token::FI) => ActionTypes::Action(34),
        (201, Token::GREATER) => ActionTypes::Action(34),
        (201, Token::GREATER_GREATER) => ActionTypes::Action(34),
        (201, Token::LESS) => ActionTypes::Action(34),
        (201, Token::NEWLINE) => ActionTypes::Action(34),
        (201, Token::OR_OR) => ActionTypes::Action(34),
        (201, Token::SEMI) => ActionTypes::Action(34),
        (201, Token::WORD(_)) => ActionTypes::Action(34),
        (202, Token::AND) => ActionTypes::Action(34),
        (202, Token::AND_AND) => ActionTypes::Action(34),
        (202, Token::BAR) => ActionTypes::Action(34),
        (202, Token::GREATER) => ActionTypes::Action(34),
        (202, Token::GREATER_GREATER) => ActionTypes::Action(34),
        (202, Token::LESS) => ActionTypes::Action(34),
        (202, Token::NEWLINE) => ActionTypes::Action(34),
        (202, Token::OR_OR) => ActionTypes::Action(34),
        (202, Token::SEMI) => ActionTypes::Action(34),
        (202, Token::THEN) => ActionTypes::Action(34),
        (202, Token::WORD(_)) => ActionTypes::Action(34),
        (203, Token::AND) => ActionTypes::Action(35),
        (203, Token::AND_AND) => ActionTypes::Action(35),
        (203, Token::BAR) => ActionTypes::Action(35),
        (203, Token::ELIF) => ActionTypes::Action(35),
        (203, Token::ELSE) => ActionTypes::Action(35),
        (203, Token::FI) => ActionTypes::Action(35),
        (203, Token::GREATER) => ActionTypes::Action(35),
        (203, Token::GREATER_GREATER) => ActionTypes::Action(35),
        (203, Token::LESS) => ActionTypes::Action(35),
        (203, Token::NEWLINE) => ActionTypes::Action(35),
        (203, Token::OR_OR) => ActionTypes::Action(35),
        (203, Token::SEMI) => ActionTypes::Action(35),
        (203, Token::WORD(_)) => ActionTypes::Action(35),
        (204, Token::AND) => ActionTypes::Action(35),
        (204, Token::AND_AND) => ActionTypes::Action(35),
        (204, Token::BAR) => ActionTypes::Action(35),
        (204, Token::EOF) => ActionTypes::Action(35),
        (204, Token::GREATER) => ActionTypes::Action(35),
        (204, Token::GREATER_GREATER) => ActionTypes::Action(35),
        (204, Token::LESS) => ActionTypes::Action(35),
        (204, Token::NEWLINE) => ActionTypes::Action(35),
        (204, Token::OR_OR) => ActionTypes::Action(35),
        (204, Token::SEMI) => ActionTypes::Action(35),
        (204, Token::WORD(_)) => ActionTypes::Action(35),
        (205, Token::AND) => ActionTypes::Action(35),
        (205, Token::AND_AND) => ActionTypes::Action(35),
        (205, Token::BAR) => ActionTypes::Action(35),
        (205, Token::FI) => ActionTypes::Action(35),
        (205, Token::GREATER) => ActionTypes::Action(35),
        (205, Token::GREATER_GREATER) => ActionTypes::Action(35),
        (205, Token::LESS) => ActionTypes::Action(35),
        (205, Token::NEWLINE) => ActionTypes::Action(35),
        (205, Token::OR_OR) => ActionTypes::Action(35),
        (205, Token::SEMI) => ActionTypes::Action(35),
        (205, Token::WORD(_)) => ActionTypes::Action(35),
        (206, Token::AND) => ActionTypes::Action(35),
        (206, Token::AND_AND) => ActionTypes::Action(35),
        (206, Token::BAR) => ActionTypes::Action(35),
        (206, Token::GREATER) => ActionTypes::Action(35),
        (206, Token::GREATER_GREATER) => ActionTypes::Action(35),
        (206, Token::LESS) => ActionTypes::Action(35),
        (206, Token::NEWLINE) => ActionTypes::Action(35),
        (206, Token::OR_OR) => ActionTypes::Action(35),
        (206, Token::SEMI) => ActionTypes::Action(35),
        (206, Token::THEN) => ActionTypes::Action(35),
        (206, Token::WORD(_)) => ActionTypes::Action(35),
        (207, Token::AND) => ActionTypes::Action(36),
        (207, Token::AND_AND) => ActionTypes::Action(36),
        (207, Token::BAR) => ActionTypes::Action(36),
        (207, Token::ELIF) => ActionTypes::Action(36),
        (207, Token::ELSE) => ActionTypes::Action(36),
        (207, Token::FI) => ActionTypes::Action(36),
        (207, Token::GREATER) => ActionTypes::Action(36),
        (207, Token::GREATER_GREATER) => ActionTypes::Action(36),
        (207, Token::LESS) => ActionTypes::Action(36),
        (207, Token::NEWLINE) => ActionTypes::Action(36),
        (207, Token::OR_OR) => ActionTypes::Action(36),
        (207, Token::SEMI) => ActionTypes::Action(36),
        (207, Token::WORD(_)) => ActionTypes::Action(36),
        (208, Token::AND) => ActionTypes::Action(36),
        (208, Token::AND_AND) => ActionTypes::Action(36),
        (208, Token::BAR) => ActionTypes::Action(36),
        (208, Token::EOF) => ActionTypes::Action(36),
        (208, Token::GREATER) => ActionTypes::Action(36),
        (208, Token::GREATER_GREATER) => ActionTypes::Action(36),
        (208, Token::LESS) => ActionTypes::Action(36),
        (208, Token::NEWLINE) => ActionTypes::Action(36),
        (208, Token::OR_OR) => ActionTypes::Action(36),
        (208, Token::SEMI) => ActionTypes::Action(36),
        (208, Token::WORD(_)) => ActionTypes::Action(36),
        (209, Token::AND) => ActionTypes::Action(36),
        (209, Token::AND_AND) => ActionTypes::Action(36),
        (209, Token::BAR) => ActionTypes::Action(36),
        (209, Token::FI) => ActionTypes::Action(36),
        (209, Token::GREATER) => ActionTypes::Action(36),
        (209, Token::GREATER_GREATER) => ActionTypes::Action(36),
        (209, Token::LESS) => ActionTypes::Action(36),
        (209, Token::NEWLINE) => ActionTypes::Action(36),
        (209, Token::OR_OR) => ActionTypes::Action(36),
        (209, Token::SEMI) => ActionTypes::Action(36),
        (209, Token::WORD(_)) => ActionTypes::Action(36),
        (210, Token::AND) => ActionTypes::Action(36),
        (210, Token::AND_AND) => ActionTypes::Action(36),
        (210, Token::BAR) => ActionTypes::Action(36),
        (210, Token::GREATER) => ActionTypes::Action(36),
        (210, Token::GREATER_GREATER) => ActionTypes::Action(36),
        (210, Token::LESS) => ActionTypes::Action(36),
        (210, Token::NEWLINE) => ActionTypes::Action(36),
        (210, Token::OR_OR) => ActionTypes::Action(36),
        (210, Token::SEMI) => ActionTypes::Action(36),
        (210, Token::THEN) => ActionTypes::Action(36),
        (210, Token::WORD(_)) => ActionTypes::Action(36),
        (211, Token::AND) => ActionTypes::Action(37),
        (211, Token::AND_AND) => ActionTypes::Action(37),
        (211, Token::BAR) => ActionTypes::Action(37),
        (211, Token::ELIF) => ActionTypes::Action(37),
        (211, Token::ELSE) => ActionTypes::Action(37),
        (211, Token::FI) => ActionTypes::Action(37),
        (211, Token::GREATER) => ActionTypes::Action(37),
        (211, Token::GREATER_GREATER) => ActionTypes::Action(37),
        (211, Token::LESS) => ActionTypes::Action(37),
        (211, Token::NEWLINE) => ActionTypes::Action(37),
        (211, Token::OR_OR) => ActionTypes::Action(37),
        (211, Token::SEMI) => ActionTypes::Action(37),
        (211, Token::WORD(_)) => ActionTypes::Action(37),
        (212, Token::AND) => ActionTypes::Action(37),
        (212, Token::AND_AND) => ActionTypes::Action(37),
        (212, Token::BAR) => ActionTypes::Action(37),
        (212, Token::EOF) => ActionTypes::Action(37),
        (212, Token::GREATER) => ActionTypes::Action(37),
        (212, Token::GREATER_GREATER) => ActionTypes::Action(37),
        (212, Token::LESS) => ActionTypes::Action(37),
        (212, Token::NEWLINE) => ActionTypes::Action(37),
        (212, Token::OR_OR) => ActionTypes::Action(37),
        (212, Token::SEMI) => ActionTypes::Action(37),
        (212, Token::WORD(_)) => ActionTypes::Action(37),
        (213, Token::AND) => ActionTypes::Action(37),
        (213, Token::AND_AND) => ActionTypes::Action(37),
        (213, Token::BAR) => ActionTypes::Action(37),
        (213, Token::FI) => ActionTypes::Action(37),
        (213, Token::GREATER) => ActionTypes::Action(37),
        (213, Token::GREATER_GREATER) => ActionTypes::Action(37),
        (213, Token::LESS) => ActionTypes::Action(37),
        (213, Token::NEWLINE) => ActionTypes::Action(37),
        (213, Token::OR_OR) => ActionTypes::Action(37),
        (213, Token::SEMI) => ActionTypes::Action(37),
        (213, Token::WORD(_)) => ActionTypes::Action(37),
        (214, Token::AND) => ActionTypes::Action(37),
        (214, Token::AND_AND) => ActionTypes::Action(37),
        (214, Token::BAR) => ActionTypes::Action(37),
        (214, Token::GREATER) => ActionTypes::Action(37),
        (214, Token::GREATER_GREATER) => ActionTypes::Action(37),
        (214, Token::LESS) => ActionTypes::Action(37),
        (214, Token::NEWLINE) => ActionTypes::Action(37),
        (214, Token::OR_OR) => ActionTypes::Action(37),
        (214, Token::SEMI) => ActionTypes::Action(37),
        (214, Token::THEN) => ActionTypes::Action(37),
        (214, Token::WORD(_)) => ActionTypes::Action(37),
        (215, Token::AND) => ActionTypes::Shift(15),
        (215, Token::AND_AND) => ActionTypes::Shift(153),
        (215, Token::EOF) => ActionTypes::Action(38),
        (215, Token::NEWLINE) => ActionTypes::Action(38),
        (215, Token::OR_OR) => ActionTypes::Shift(154),
        (215, Token::SEMI) => ActionTypes::Shift(16),
        (216, Token::AND) => ActionTypes::Action(39),
        (216, Token::AND_AND) => ActionTypes::Action(39),
        (216, Token::EOF) => ActionTypes::Action(39),
        (216, Token::NEWLINE) => ActionTypes::Action(39),
        (216, Token::OR_OR) => ActionTypes::Action(39),
        (216, Token::SEMI) => ActionTypes::Action(39),
        (217, Token::AND) => ActionTypes::Action(40),
        (217, Token::AND_AND) => ActionTypes::Shift(153),
        (217, Token::EOF) => ActionTypes::Action(40),
        (217, Token::NEWLINE) => ActionTypes::Action(40),
        (217, Token::OR_OR) => ActionTypes::Shift(154),
        (217, Token::SEMI) => ActionTypes::Shift(18),
        (218, Token::AND) => ActionTypes::Action(41),
        (218, Token::AND_AND) => ActionTypes::Action(41),
        (218, Token::EOF) => ActionTypes::Action(41),
        (218, Token::NEWLINE) => ActionTypes::Action(41),
        (218, Token::OR_OR) => ActionTypes::Action(41),
        (218, Token::SEMI) => ActionTypes::Action(41),
        (219, Token::AND) => ActionTypes::Action(42),
        (219, Token::AND_AND) => ActionTypes::Action(42),
        (219, Token::EOF) => ActionTypes::Action(42),
        (219, Token::NEWLINE) => ActionTypes::Action(42),
        (219, Token::OR_OR) => ActionTypes::Action(42),
        (219, Token::SEMI) => ActionTypes::Action(42),
        (220, Token::AND) => ActionTypes::Action(43),
        (220, Token::AND_AND) => ActionTypes::Shift(153),
        (220, Token::EOF) => ActionTypes::Action(43),
        (220, Token::NEWLINE) => ActionTypes::Action(43),
        (220, Token::OR_OR) => ActionTypes::Shift(154),
        (220, Token::SEMI) => ActionTypes::Action(43),
        (221, Token::EOF) => ActionTypes::Action(44),
        (222, Token::EOF) => ActionTypes::Action(45),
        _ => ActionTypes::Failure,
    }
}

fn reduce(num: usize, mut list_etat: Vec<usize>, mut pile: Vec<RulesType>) -> ReturnReduce {
    match num {
        21 => {
            let l1 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let l2 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..4 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST1({
                command::Command::Connection(Box::new(l1), command::Connector::And, Box::new(l2))
            }));
            ReturnReduce::Success(list_etat, pile, "list1")
        }
        34 => {
            let sc = match pile.pop() {
                Some(RulesType::SIMPLE_COMMAND(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let sce = match pile.pop() {
                Some(RulesType::SIMPLE_COMMAND_ELEMENT(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_COMMAND({ make_simple_command(sce, sc) }));
            ReturnReduce::Success(list_etat, pile, "simple_command")
        }
        13 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl1 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl2 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl3 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..7 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::IF_COMMAND({
                command::Command::If(Box::new(cl1), Box::new(cl2), Box::new(cl3))
            }));
            ReturnReduce::Success(list_etat, pile, "if_command")
        }
        20 => {
            let l1 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let l2 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..4 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST1({
                command::Command::Connection(
                    Box::new(l1.make_async()),
                    command::Connector::Seq,
                    Box::new(l2),
                )
            }));
            ReturnReduce::Success(list_etat, pile, "list1")
        }
        33 => {
            let ifc = match pile.pop() {
                Some(RulesType::IF_COMMAND(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SHELL_COMMAND({ ifc }));
            ReturnReduce::Success(list_etat, pile, "shell_command")
        }
        42 => {
            let sl1 = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let sl2 = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..4 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST1({
                command::Command::Connection(Box::new(sl1), command::Connector::Or, Box::new(sl2))
            }));
            ReturnReduce::Success(list_etat, pile, "simple_list1")
        }
        26 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::NEWLINE_LIST({ my_unit::MyUnit }));
            ReturnReduce::Success(list_etat, pile, "newline_list")
        }
        15 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl1 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl2 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let ec = match pile.pop() {
                Some(RulesType::ELIF_CLAUSE(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..6 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::IF_COMMAND({
                command::Command::If(Box::new(cl1), Box::new(cl2), Box::new(ec))
            }));
            ReturnReduce::Success(list_etat, pile, "if_command")
        }
        45 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST_TERMINATOR({ my_unit::MyUnit }));
            ReturnReduce::Success(list_etat, pile, "simple_list_terminator")
        }
        36 => {
            let w = match pile.pop() {
                Some(RulesType::Tok(Token::WORD(t))) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_COMMAND_ELEMENT({
                SimpleCommandElement::EltWord(w)
            }));
            ReturnReduce::Success(list_etat, pile, "simple_command_element")
        }
        23 => {
            let l1 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let l2 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..4 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST1({
                command::Command::Connection(Box::new(l1), command::Connector::Or, Box::new(l2))
            }));
            ReturnReduce::Success(list_etat, pile, "list1")
        }
        43 => {
            let sl1 = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let sl2 = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..3 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST1({
                command::Command::Connection(Box::new(sl1), command::Connector::Seq, Box::new(sl2))
            }));
            ReturnReduce::Success(list_etat, pile, "simple_list1")
        }
        0 => {
            let l = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..3 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST0({ l.make_async() }));
            ReturnReduce::Success(list_etat, pile, "list0")
        }
        4 => {
            let sl = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST({ sl }));
            ReturnReduce::Success(list_etat, pile, "simple_list")
        }
        28 => {
            let p1 = match pile.pop() {
                Some(RulesType::PIPELINE(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let p2 = match pile.pop() {
                Some(RulesType::PIPELINE(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..4 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::PIPELINE({
                command::Command::Connection(Box::new(p1), command::Connector::Pipe, Box::new(p2))
            }));
            ReturnReduce::Success(list_etat, pile, "pipeline")
        }
        31 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let w = match pile.pop() {
                Some(RulesType::Tok(Token::WORD(t))) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::REDIRECTION({
                command::Redirect::OutputAppend(w)
            }));
            ReturnReduce::Success(list_etat, pile, "redirection")
        }
        38 => {
            let sl = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST({ sl }));
            ReturnReduce::Success(list_etat, pile, "simple_list")
        }
        24 => {
            let l1 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let l2 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..4 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST1({
                command::Command::Connection(Box::new(l1), command::Connector::Seq, Box::new(l2))
            }));
            ReturnReduce::Success(list_etat, pile, "list1")
        }
        8 => {
            let l = match pile.pop() {
                Some(RulesType::LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::COMPOUND_LIST({ l }));
            ReturnReduce::Success(list_etat, pile, "compound_list")
        }
        18 => {
            let sl = match pile.pop() {
                Some(RulesType::SIMPLE_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::INPUTUNIT({ sl }));
            ReturnReduce::Success(list_etat, pile, "inputunit")
        }
        40 => {
            let sl1 = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let sl2 = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..3 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST1({
                command::Command::Connection(
                    Box::new(sl1.make_async()),
                    command::Connector::Seq,
                    Box::new(sl2),
                )
            }));
            ReturnReduce::Success(list_etat, pile, "simple_list1")
        }
        16 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::INPUTUNIT({ command::Command::Nothing }));
            ReturnReduce::Success(list_etat, pile, "inputunit")
        }
        19 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let l = match pile.pop() {
                Some(RulesType::LIST0(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST({ l }));
            ReturnReduce::Success(list_etat, pile, "list")
        }
        3 => {
            let sl = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST({ sl.make_async() }));
            ReturnReduce::Success(list_etat, pile, "simple_list")
        }
        7 => {
            for _i in 0..0 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::NEWLINE_LIST({ my_unit::MyUnit }));
            ReturnReduce::Success(list_etat, pile, "newline_list")
        }
        27 => {
            let c = match pile.pop() {
                Some(RulesType::COMMAND(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::PIPELINE({ c }));
            ReturnReduce::Success(list_etat, pile, "pipeline")
        }
        10 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl1 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl2 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..4 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::ELIF_CLAUSE({
                command::Command::If(
                    Box::new(cl1),
                    Box::new(cl2),
                    Box::new(command::Command::Nothing),
                )
            }));
            ReturnReduce::Success(list_etat, pile, "elif_clause")
        }
        2 => {
            let l = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..3 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST0({ l }));
            ReturnReduce::Success(list_etat, pile, "list0")
        }
        17 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::INPUTUNIT({ command::Command::Nothing }));
            ReturnReduce::Success(list_etat, pile, "inputunit")
        }
        12 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl1 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl2 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let ec = match pile.pop() {
                Some(RulesType::ELIF_CLAUSE(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..5 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::ELIF_CLAUSE({
                command::Command::If(Box::new(cl1), Box::new(cl2), Box::new(ec))
            }));
            ReturnReduce::Success(list_etat, pile, "elif_clause")
        }
        32 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let w = match pile.pop() {
                Some(RulesType::Tok(Token::WORD(t))) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::REDIRECTION({ command::Redirect::Input(w) }));
            ReturnReduce::Success(list_etat, pile, "redirection")
        }
        37 => {
            let r = match pile.pop() {
                Some(RulesType::REDIRECTION(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_COMMAND_ELEMENT({
                SimpleCommandElement::EltRedi(r)
            }));
            ReturnReduce::Success(list_etat, pile, "simple_command_element")
        }
        35 => {
            let sce = match pile.pop() {
                Some(RulesType::SIMPLE_COMMAND_ELEMENT(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_COMMAND({
                match sce {
                    SimpleCommandElement::EltWord(w) => command::SimpleCommand {
                        cmd_line: Vec::from([w]),
                        cmd_redirects: Vec::new(),
                        cmd_bg: false,
                    },
                    SimpleCommandElement::EltRedi(r) => command::SimpleCommand {
                        cmd_line: Vec::new(),
                        cmd_redirects: Vec::from([r]),
                        cmd_bg: false,
                    },
                }
            }));
            ReturnReduce::Success(list_etat, pile, "simple_command")
        }
        14 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl1 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl2 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..5 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::IF_COMMAND({
                command::Command::If(
                    Box::new(cl1),
                    Box::new(cl2),
                    Box::new(command::Command::Nothing),
                )
            }));
            ReturnReduce::Success(list_etat, pile, "if_command")
        }
        22 => {
            let l1 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let l2 = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..4 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST1({
                command::Command::Connection(Box::new(l1), command::Connector::Seq, Box::new(l2))
            }));
            ReturnReduce::Success(list_etat, pile, "list1")
        }
        11 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl1 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl2 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let cl3 = match pile.pop() {
                Some(RulesType::COMPOUND_LIST(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..6 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::ELIF_CLAUSE({
                command::Command::If(Box::new(cl1), Box::new(cl2), Box::new(cl3))
            }));
            ReturnReduce::Success(list_etat, pile, "elif_clause")
        }
        1 => {
            let l = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..3 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST0({ l }));
            ReturnReduce::Success(list_etat, pile, "list0")
        }
        25 => {
            let pc = match pile.pop() {
                Some(RulesType::PIPELINE_COMMAND(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::LIST1({ pc }));
            ReturnReduce::Success(list_etat, pile, "list1")
        }
        41 => {
            let sl1 = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let sl2 = match pile.pop() {
                Some(RulesType::SIMPLE_LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..4 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST1({
                command::Command::Connection(Box::new(sl1), command::Connector::And, Box::new(sl2))
            }));
            ReturnReduce::Success(list_etat, pile, "simple_list1")
        }
        6 => {
            let sc = match pile.pop() {
                Some(RulesType::SIMPLE_COMMAND(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::COMMAND({
                command::Command::SimpleCommand(clean_simple_command(sc))
            }));
            ReturnReduce::Success(list_etat, pile, "command")
        }
        44 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST_TERMINATOR({ my_unit::MyUnit }));
            ReturnReduce::Success(list_etat, pile, "simple_list_terminator")
        }
        29 => {
            let p = match pile.pop() {
                Some(RulesType::PIPELINE(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::PIPELINE_COMMAND({ p }));
            ReturnReduce::Success(list_etat, pile, "pipeline_command")
        }
        9 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => return ReturnReduce::SamenhirErrorReduce,
                Some(_) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let l = match pile.pop() {
                Some(RulesType::LIST1(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::COMPOUND_LIST({ l }));
            ReturnReduce::Success(list_etat, pile, "compound_list")
        }
        30 => {
            match pile.pop() {
                Some(RulesType::Tok(_)) => (),
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            let w = match pile.pop() {
                Some(RulesType::Tok(Token::WORD(t))) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..2 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::REDIRECTION({ command::Redirect::Output(w) }));
            ReturnReduce::Success(list_etat, pile, "redirection")
        }
        39 => {
            let pc = match pile.pop() {
                Some(RulesType::PIPELINE_COMMAND(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::SIMPLE_LIST1({ pc }));
            ReturnReduce::Success(list_etat, pile, "simple_list1")
        }
        5 => {
            let shc = match pile.pop() {
                Some(RulesType::SHELL_COMMAND(t)) => t,
                _ => return ReturnReduce::SamenhirErrorReduce,
            };
            for _i in 0..1 {
                match list_etat.pop() {
                    Some(_) => (),
                    _ => return ReturnReduce::SamenhirErrorReduce,
                }
            }
            pile.push(RulesType::COMMAND({
                unsafe { FIRST_WORD = true };
                shc
            }));
            ReturnReduce::Success(list_etat, pile, "command")
        }
        _ => ReturnReduce::SamenhirErrorReduce,
    }
}

fn goto(i: usize, read_rule: &str) -> isize {
    match (i, read_rule) {
        (1, "command") => 159,
        (1, "if_command") => 195,
        (1, "list0") => 104,
        (1, "list1") => 65,
        (1, "pipeline") => 164,
        (1, "pipeline_command") => 146,
        (1, "redirection") => 211,
        (1, "shell_command") => 39,
        (1, "simple_command") => 43,
        (1, "simple_command_element") => 203,
        (2, "command") => 159,
        (2, "if_command") => 195,
        (2, "list1") => 116,
        (2, "pipeline") => 164,
        (2, "pipeline_command") => 146,
        (2, "redirection") => 211,
        (2, "shell_command") => 39,
        (2, "simple_command") => 43,
        (2, "simple_command_element") => 203,
        (3, "command") => 159,
        (3, "if_command") => 195,
        (3, "list1") => 118,
        (3, "pipeline") => 164,
        (3, "pipeline_command") => 146,
        (3, "redirection") => 211,
        (3, "shell_command") => 39,
        (3, "simple_command") => 43,
        (3, "simple_command_element") => 203,
        (4, "command") => 159,
        (4, "if_command") => 195,
        (4, "list1") => 120,
        (4, "pipeline") => 164,
        (4, "pipeline_command") => 146,
        (4, "redirection") => 211,
        (4, "shell_command") => 39,
        (4, "simple_command") => 43,
        (4, "simple_command_element") => 203,
        (5, "command") => 159,
        (5, "if_command") => 195,
        (5, "list1") => 116,
        (5, "pipeline") => 164,
        (5, "pipeline_command") => 146,
        (5, "redirection") => 211,
        (5, "shell_command") => 39,
        (5, "simple_command") => 43,
        (5, "simple_command_element") => 203,
        (6, "command") => 159,
        (6, "if_command") => 195,
        (6, "list1") => 117,
        (6, "pipeline") => 164,
        (6, "pipeline_command") => 146,
        (6, "redirection") => 211,
        (6, "shell_command") => 39,
        (6, "simple_command") => 43,
        (6, "simple_command_element") => 203,
        (7, "command") => 159,
        (7, "if_command") => 195,
        (7, "list1") => 118,
        (7, "pipeline") => 164,
        (7, "pipeline_command") => 146,
        (7, "redirection") => 211,
        (7, "shell_command") => 39,
        (7, "simple_command") => 43,
        (7, "simple_command_element") => 203,
        (8, "command") => 159,
        (8, "if_command") => 195,
        (8, "list1") => 119,
        (8, "pipeline") => 164,
        (8, "pipeline_command") => 146,
        (8, "redirection") => 211,
        (8, "shell_command") => 39,
        (8, "simple_command") => 43,
        (8, "simple_command_element") => 203,
        (9, "command") => 159,
        (9, "if_command") => 195,
        (9, "list1") => 120,
        (9, "pipeline") => 164,
        (9, "pipeline_command") => 146,
        (9, "redirection") => 211,
        (9, "shell_command") => 39,
        (9, "simple_command") => 43,
        (9, "simple_command_element") => 203,
        (10, "command") => 159,
        (10, "if_command") => 195,
        (10, "pipeline") => 163,
        (10, "redirection") => 211,
        (10, "shell_command") => 39,
        (10, "simple_command") => 43,
        (10, "simple_command_element") => 203,
        (11, "command") => 160,
        (11, "if_command") => 196,
        (11, "pipeline") => 166,
        (11, "pipeline_command") => 216,
        (11, "redirection") => 212,
        (11, "shell_command") => 40,
        (11, "simple_command") => 44,
        (11, "simple_command_element") => 204,
        (11, "simple_list") => 102,
        (11, "simple_list1") => 215,
        (12, "command") => 160,
        (12, "if_command") => 196,
        (12, "pipeline") => 165,
        (12, "redirection") => 212,
        (12, "shell_command") => 40,
        (12, "simple_command") => 44,
        (12, "simple_command_element") => 204,
        (13, "command") => 160,
        (13, "if_command") => 196,
        (13, "pipeline") => 166,
        (13, "pipeline_command") => 216,
        (13, "redirection") => 212,
        (13, "shell_command") => 40,
        (13, "simple_command") => 44,
        (13, "simple_command_element") => 204,
        (13, "simple_list1") => 218,
        (14, "command") => 160,
        (14, "if_command") => 196,
        (14, "pipeline") => 166,
        (14, "pipeline_command") => 216,
        (14, "redirection") => 212,
        (14, "shell_command") => 40,
        (14, "simple_command") => 44,
        (14, "simple_command_element") => 204,
        (14, "simple_list1") => 219,
        (15, "command") => 160,
        (15, "if_command") => 196,
        (15, "pipeline") => 166,
        (15, "pipeline_command") => 216,
        (15, "redirection") => 212,
        (15, "shell_command") => 40,
        (15, "simple_command") => 44,
        (15, "simple_command_element") => 204,
        (15, "simple_list1") => 217,
        (16, "command") => 160,
        (16, "if_command") => 196,
        (16, "pipeline") => 166,
        (16, "pipeline_command") => 216,
        (16, "redirection") => 212,
        (16, "shell_command") => 40,
        (16, "simple_command") => 44,
        (16, "simple_command_element") => 204,
        (16, "simple_list1") => 220,
        (17, "command") => 160,
        (17, "if_command") => 196,
        (17, "pipeline") => 166,
        (17, "pipeline_command") => 216,
        (17, "redirection") => 212,
        (17, "shell_command") => 40,
        (17, "simple_command") => 44,
        (17, "simple_command_element") => 204,
        (17, "simple_list1") => 217,
        (18, "command") => 160,
        (18, "if_command") => 196,
        (18, "pipeline") => 166,
        (18, "pipeline_command") => 216,
        (18, "redirection") => 212,
        (18, "shell_command") => 40,
        (18, "simple_command") => 44,
        (18, "simple_command_element") => 204,
        (18, "simple_list1") => 220,
        (19, "command") => 161,
        (19, "if_command") => 197,
        (19, "list0") => 105,
        (19, "list1") => 66,
        (19, "pipeline") => 168,
        (19, "pipeline_command") => 147,
        (19, "redirection") => 213,
        (19, "shell_command") => 41,
        (19, "simple_command") => 45,
        (19, "simple_command_element") => 205,
        (20, "command") => 161,
        (20, "if_command") => 197,
        (20, "list1") => 121,
        (20, "pipeline") => 168,
        (20, "pipeline_command") => 147,
        (20, "redirection") => 213,
        (20, "shell_command") => 41,
        (20, "simple_command") => 45,
        (20, "simple_command_element") => 205,
        (21, "command") => 161,
        (21, "if_command") => 197,
        (21, "list1") => 123,
        (21, "pipeline") => 168,
        (21, "pipeline_command") => 147,
        (21, "redirection") => 213,
        (21, "shell_command") => 41,
        (21, "simple_command") => 45,
        (21, "simple_command_element") => 205,
        (22, "command") => 161,
        (22, "if_command") => 197,
        (22, "list1") => 125,
        (22, "pipeline") => 168,
        (22, "pipeline_command") => 147,
        (22, "redirection") => 213,
        (22, "shell_command") => 41,
        (22, "simple_command") => 45,
        (22, "simple_command_element") => 205,
        (23, "command") => 161,
        (23, "if_command") => 197,
        (23, "list1") => 121,
        (23, "pipeline") => 168,
        (23, "pipeline_command") => 147,
        (23, "redirection") => 213,
        (23, "shell_command") => 41,
        (23, "simple_command") => 45,
        (23, "simple_command_element") => 205,
        (24, "command") => 161,
        (24, "if_command") => 197,
        (24, "list1") => 122,
        (24, "pipeline") => 168,
        (24, "pipeline_command") => 147,
        (24, "redirection") => 213,
        (24, "shell_command") => 41,
        (24, "simple_command") => 45,
        (24, "simple_command_element") => 205,
        (25, "command") => 161,
        (25, "if_command") => 197,
        (25, "list1") => 123,
        (25, "pipeline") => 168,
        (25, "pipeline_command") => 147,
        (25, "redirection") => 213,
        (25, "shell_command") => 41,
        (25, "simple_command") => 45,
        (25, "simple_command_element") => 205,
        (26, "command") => 161,
        (26, "if_command") => 197,
        (26, "list1") => 124,
        (26, "pipeline") => 168,
        (26, "pipeline_command") => 147,
        (26, "redirection") => 213,
        (26, "shell_command") => 41,
        (26, "simple_command") => 45,
        (26, "simple_command_element") => 205,
        (27, "command") => 161,
        (27, "if_command") => 197,
        (27, "list1") => 125,
        (27, "pipeline") => 168,
        (27, "pipeline_command") => 147,
        (27, "redirection") => 213,
        (27, "shell_command") => 41,
        (27, "simple_command") => 45,
        (27, "simple_command_element") => 205,
        (28, "command") => 161,
        (28, "if_command") => 197,
        (28, "pipeline") => 167,
        (28, "redirection") => 213,
        (28, "shell_command") => 41,
        (28, "simple_command") => 45,
        (28, "simple_command_element") => 205,
        (29, "command") => 162,
        (29, "if_command") => 198,
        (29, "list0") => 106,
        (29, "list1") => 67,
        (29, "pipeline") => 170,
        (29, "pipeline_command") => 148,
        (29, "redirection") => 214,
        (29, "shell_command") => 42,
        (29, "simple_command") => 46,
        (29, "simple_command_element") => 206,
        (30, "command") => 162,
        (30, "if_command") => 198,
        (30, "list1") => 126,
        (30, "pipeline") => 170,
        (30, "pipeline_command") => 148,
        (30, "redirection") => 214,
        (30, "shell_command") => 42,
        (30, "simple_command") => 46,
        (30, "simple_command_element") => 206,
        (31, "command") => 162,
        (31, "if_command") => 198,
        (31, "list1") => 128,
        (31, "pipeline") => 170,
        (31, "pipeline_command") => 148,
        (31, "redirection") => 214,
        (31, "shell_command") => 42,
        (31, "simple_command") => 46,
        (31, "simple_command_element") => 206,
        (32, "command") => 162,
        (32, "if_command") => 198,
        (32, "list1") => 130,
        (32, "pipeline") => 170,
        (32, "pipeline_command") => 148,
        (32, "redirection") => 214,
        (32, "shell_command") => 42,
        (32, "simple_command") => 46,
        (32, "simple_command_element") => 206,
        (33, "command") => 162,
        (33, "if_command") => 198,
        (33, "list1") => 126,
        (33, "pipeline") => 170,
        (33, "pipeline_command") => 148,
        (33, "redirection") => 214,
        (33, "shell_command") => 42,
        (33, "simple_command") => 46,
        (33, "simple_command_element") => 206,
        (34, "command") => 162,
        (34, "if_command") => 198,
        (34, "list1") => 127,
        (34, "pipeline") => 170,
        (34, "pipeline_command") => 148,
        (34, "redirection") => 214,
        (34, "shell_command") => 42,
        (34, "simple_command") => 46,
        (34, "simple_command_element") => 206,
        (35, "command") => 162,
        (35, "if_command") => 198,
        (35, "list1") => 128,
        (35, "pipeline") => 170,
        (35, "pipeline_command") => 148,
        (35, "redirection") => 214,
        (35, "shell_command") => 42,
        (35, "simple_command") => 46,
        (35, "simple_command_element") => 206,
        (36, "command") => 162,
        (36, "if_command") => 198,
        (36, "list1") => 129,
        (36, "pipeline") => 170,
        (36, "pipeline_command") => 148,
        (36, "redirection") => 214,
        (36, "shell_command") => 42,
        (36, "simple_command") => 46,
        (36, "simple_command_element") => 206,
        (37, "command") => 162,
        (37, "if_command") => 198,
        (37, "list1") => 130,
        (37, "pipeline") => 170,
        (37, "pipeline_command") => 148,
        (37, "redirection") => 214,
        (37, "shell_command") => 42,
        (37, "simple_command") => 46,
        (37, "simple_command_element") => 206,
        (38, "command") => 162,
        (38, "if_command") => 198,
        (38, "pipeline") => 169,
        (38, "redirection") => 214,
        (38, "shell_command") => 42,
        (38, "simple_command") => 46,
        (38, "simple_command_element") => 206,
        (43, "redirection") => 211,
        (43, "simple_command_element") => 199,
        (44, "redirection") => 212,
        (44, "simple_command_element") => 200,
        (45, "redirection") => 213,
        (45, "simple_command_element") => 201,
        (46, "redirection") => 214,
        (46, "simple_command_element") => 202,
        (47, "compound_list") => 68,
        (47, "list") => 62,
        (47, "newline_list") => 1,
        (48, "compound_list") => 69,
        (48, "list") => 62,
        (48, "newline_list") => 1,
        (49, "compound_list") => 70,
        (49, "list") => 62,
        (49, "newline_list") => 1,
        (50, "compound_list") => 71,
        (50, "list") => 62,
        (50, "newline_list") => 1,
        (51, "compound_list") => 72,
        (51, "list") => 62,
        (51, "newline_list") => 1,
        (52, "compound_list") => 74,
        (52, "list") => 63,
        (52, "newline_list") => 19,
        (53, "compound_list") => 80,
        (53, "list") => 63,
        (53, "newline_list") => 19,
        (54, "compound_list") => 81,
        (54, "list") => 63,
        (54, "newline_list") => 19,
        (55, "compound_list") => 82,
        (55, "list") => 63,
        (55, "newline_list") => 19,
        (56, "compound_list") => 83,
        (56, "list") => 63,
        (56, "newline_list") => 19,
        (57, "compound_list") => 73,
        (57, "list") => 64,
        (57, "newline_list") => 29,
        (58, "compound_list") => 76,
        (58, "list") => 64,
        (58, "newline_list") => 29,
        (59, "compound_list") => 77,
        (59, "list") => 64,
        (59, "newline_list") => 29,
        (60, "compound_list") => 78,
        (60, "list") => 64,
        (60, "newline_list") => 29,
        (61, "compound_list") => 79,
        (61, "list") => 64,
        (61, "newline_list") => 29,
        (68, "elif_clause") => 75,
        (69, "elif_clause") => 92,
        (70, "elif_clause") => 93,
        (71, "elif_clause") => 94,
        (72, "elif_clause") => 95,
        (102, "simple_list_terminator") => 103,
        (107, "newline_list") => 2,
        (108, "newline_list") => 20,
        (109, "newline_list") => 30,
        (110, "newline_list") => 3,
        (111, "newline_list") => 21,
        (112, "newline_list") => 31,
        (113, "newline_list") => 4,
        (114, "newline_list") => 22,
        (115, "newline_list") => 32,
        (131, "newline_list") => 5,
        (132, "newline_list") => 23,
        (133, "newline_list") => 33,
        (134, "newline_list") => 6,
        (135, "newline_list") => 24,
        (136, "newline_list") => 34,
        (137, "newline_list") => 7,
        (138, "newline_list") => 25,
        (139, "newline_list") => 35,
        (140, "newline_list") => 8,
        (141, "newline_list") => 26,
        (142, "newline_list") => 36,
        (143, "newline_list") => 9,
        (144, "newline_list") => 27,
        (145, "newline_list") => 37,
        (149, "newline_list") => 10,
        (150, "newline_list") => 12,
        (151, "newline_list") => 28,
        (152, "newline_list") => 38,
        (153, "newline_list") => 13,
        (154, "newline_list") => 14,
        _ => -1,
    }
}

fn action_all(
    new_token: fn() -> Result<Token, &'static str>,
    mut etat: usize,
) -> Result<command::Command, Errors> {
    let mut pile = Vec::<RulesType>::new();
    let mut liste_etats = Vec::<usize>::new();
    let mut next_token = match new_token() {
        Ok(t) => t,
        Err(s) => return Err(Errors::LexingError(s)),
    };
    loop {
        match action_table(etat, &next_token) {
            ActionTypes::Success => return Err(Errors::ParsingError),
            ActionTypes::Action(i) => {
                liste_etats.push(etat);
                let s = reduce(i, liste_etats, pile);
                let nom = match s {
                    ReturnReduce::SamenhirErrorReduce => return Err(Errors::ParsingError),
                    ReturnReduce::Success(e, p, n) => {
                        pile = p;
                        liste_etats = e;
                        n
                    }
                };
                let l2 = liste_etats.len();
                let i = goto(liste_etats[l2 - 1], nom);
                if i == -1 {
                    if pile.len() != 1 {
                        return Err(Errors::ParsingError);
                    }
                    match pile.swap_remove(0) {
                        RulesType::INPUTUNIT(t) => return Ok(t),
                        _ => return Err(Errors::ParsingError),
                    }
                }
                etat = i as usize;
            }
            ActionTypes::Shift(i) => {
                liste_etats.push(etat);
                pile.push(RulesType::Tok(next_token));
                etat = i;
                next_token = match new_token() {
                    Ok(t) => t,
                    Err(s) => return Err(Errors::LexingError(s)),
                };
            }
            ActionTypes::Failure => return Err(Errors::ParsingError),
        }
    }
}

pub fn inputunit(lexer: fn() -> Result<Token, &'static str>) -> Result<command::Command, Errors> {
    action_all(lexer, 11)
}
