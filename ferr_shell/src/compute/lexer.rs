
use alloc::string::String;

pub struct Lexbuf {
  pos_b_lnum : usize,
  pos_b_cnum : usize,
  pos_e_cnum : usize,
  current : usize,
  read : usize,
  text : String,
  len : usize,
}

impl Lexbuf {
  fn newline(&mut self) {
    self.pos_b_lnum = self.pos_b_lnum + 1;
  }

  fn incr_pos(&mut self) -> &mut Self {
    self.current = self.current + 1;
    self
  }

  pub fn new(text : String) -> Self {
    Lexbuf {
      pos_b_lnum : 1,
      pos_b_cnum : 0,
      pos_e_cnum : 0,
      current : 0,
      read : 0,
      len : text.len(),
      text : text,
    }
  }

  fn length(&self) -> usize {
    self.len
  }

  fn reset_current(&mut self) { self.current = self.pos_e_cnum; }

  fn get_read(&mut self) -> usize { self.read }
  fn set_read(&mut self, i : usize) {
    self.pos_e_cnum = self.current;
    self.read = i;
  }


  fn next_char(&mut self) -> Result<u8, u8> {
    if self.len <= self.current {
      Err(0)
    } else {
      Ok(self.text.chars().nth(self.current).unwrap() as u8)
    }
  }


  fn finished(&mut self) -> bool {
    self.len == self.current
  }

  fn new_extremity(&mut self) {
    self.pos_b_cnum = self.pos_e_cnum
  }

  fn get_token(&self) -> String {
    let mut s = String::new();
    for i in self.pos_b_cnum..self.pos_e_cnum {
      s.push(self.text.chars().nth(i).unwrap());
    }
    s
  }

}


use super::command;
use super::parser;
use super::parser::FIRST_WORD;

pub enum LexicalErr {
    UnclosedComment,
}

pub struct LexicalError(pub LexicalErr);

fn keyword_of_string(str: &str) -> Option<parser::Token> {
    unsafe {
        FIRST_WORD = false
    };

    match str {
        "if" => Some(parser::Token::IF),
        "then" => Some(parser::Token::THEN),
        "else" => Some(parser::Token::ELSE),
        "elif" => Some(parser::Token::ELIF),
        "fi" => Some(parser::Token::FI),
        _ => None,
    }
}


/*  let newline lexbuf =
    let pos = lexbuf.lex_curr_p in
    lexbuf.lex_curr_p <-
      { pos with pos_lnum = pos.pos_lnum + 1; pos_bol = pos.pos_cnum }
*/


pub fn token (lexbuf: &mut Lexbuf) -> Result<parser::Token, &'static str> { 
	if lexbuf.finished() {
		  Ok(parser::Token::EOF)  
	} else {
    lexbuf.new_extremity();
    match token_16(lexbuf) {
      _ => {
        let i = lexbuf.get_read();
        lexbuf.reset_current();
        lexbuf.set_read(0);
        if lexbuf.pos_e_cnum <= lexbuf.pos_b_cnum { Err("Reached End of File without any corrresponding rule") }
		else if i == 15 {  comment(lexbuf); token(lexbuf)  }
		else if i == 17 {  lexbuf.newline(); Ok(parser::Token::NEWLINE)  }
		else if i == 20 {  token(lexbuf) }
		else if i == 22 {  Ok(parser::Token::LESS)  }
		else if i == 24 {  Ok(parser::Token::GREATER)  }
		else if i == 27 {  Ok(parser::Token::GREATER_GREATER)  }
		else if i == 29 {  Ok(parser::Token::AND)  }
		else if i == 32 {  Ok(parser::Token::AND_AND)  }
		else if i == 35 {  Ok(parser::Token::OR_OR)  }
		else if i == 37 {  Ok(parser::Token::BAR)  }
		else if i == 39 {  Ok(parser::Token::SEMI)  }
		else if i == 46 { let s = lexbuf.get_token(); 
        match (unsafe { FIRST_WORD }, keyword_of_string(&s)) {
        (true, Some(kw)) => Ok(kw),
        _ => {
            let mut lb = Lexbuf::new(s);
            word(&mut lb)
            }
        }
       }
		else if i == 48 { 
        double_quote(lexbuf)
       }
		else if i == 50 {  panic!("error") }
		else { Err("Undefined rule, should not happen, please report this")
		}
		}
	}
	}
}
fn token_0(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(15);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 92 { token_13(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||chr <= 8 { token_14(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_1(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(17);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn token_2(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(20);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn token_3(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(22);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn token_4(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(24);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 62 { token_5(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_5(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(27);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn token_6(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(29);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 38 { token_7(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_7(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(32);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn token_8(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(35);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn token_9(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(37);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 124 { token_8(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_10(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(39);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn token_11(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(46);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 92 { token_13(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||chr <= 8 { token_14(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_12(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(46);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 92 { token_13(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||chr <= 8 { token_14(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_13(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(46);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 92 { token_13(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||chr <= 8 { token_14(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_14(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(46);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 92 { token_13(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||chr <= 8 { token_14(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_15(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(48);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn token_16(lexbuf : &mut Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if chr == 35 { token_0(lexbuf.incr_pos()) } else
	if chr == 10 { token_1(lexbuf.incr_pos()) } else
	if chr == 32||chr == 9 { token_2(lexbuf.incr_pos()) } else
	if chr == 60 { token_3(lexbuf.incr_pos()) } else
	if chr == 62 { token_4(lexbuf.incr_pos()) } else
	if chr == 38 { token_6(lexbuf.incr_pos()) } else
	if chr == 124 { token_9(lexbuf.incr_pos()) } else
	if chr == 59 { token_10(lexbuf.incr_pos()) } else
	if chr == 92 { token_11(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(36 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||chr <= 8 { token_12(lexbuf.incr_pos()) } else
	if chr == 34 { token_15(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
pub fn word (lexbuf: &mut Lexbuf) -> Result<parser::Token, &'static str> { 
	if lexbuf.finished() {
		  Ok(parser::Token::WORD(String::new()))  
	} else {
    lexbuf.new_extremity();
    match word_5(lexbuf) {
      _ => {
        let i = lexbuf.get_read();
        lexbuf.reset_current();
        lexbuf.set_read(0);
        if lexbuf.pos_e_cnum <= lexbuf.pos_b_cnum { Err("Reached End of File without any corrresponding rule") }
		else if i == 52 {  panic!("Error")  }
		else if i == 54 {  
        let mut s1 = match double_quote(lexbuf) {
            Ok(parser::Token::WORD(s)) => s,
            _ => panic!("should not happen"),
        };
        match word(lexbuf) {
            Ok(parser::Token::WORD(s)) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        Ok(parser::Token::WORD(s1))
       }
		else if i == 57 { let c = lexbuf.get_token();  let mut s1 = String::from(c.chars().nth(1).unwrap());
        match word(lexbuf) {
            Ok(parser::Token::WORD(s)) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        Ok(parser::Token::WORD(s1))
        
     }
		else if i == 59 { let c = lexbuf.get_token(); 
        let mut s1 = String::from(c.chars().nth(0).unwrap());
        match word(lexbuf) {
            Ok(parser::Token::WORD(s)) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        Ok(parser::Token::WORD(s1))
       }
		else { Err("Undefined rule, should not happen, please report this")
		}
		}
	}
	}
}
fn word_0(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(52);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn word_1(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(54);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn word_2(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(57);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn word_3(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(59);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn word_4(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(59);
	if let Ok(chr) = lexbuf.next_char() {
	if chr <= 255 { word_2(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn word_5(lexbuf : &mut Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if chr == 124||chr == 62||(59 <= chr && chr <= 60)||chr == 38||chr == 32||(9 <= chr && chr <= 10) { word_0(lexbuf.incr_pos()) } else
	if chr == 34 { word_1(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||chr <= 8 { word_3(lexbuf.incr_pos()) } else
	if chr == 92 { word_4(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
pub fn double_quote (lexbuf: &mut Lexbuf) -> Result<parser::Token, &'static str> { 
	if lexbuf.finished() {
		 Err("end of string not handled") 
	} else {
    lexbuf.new_extremity();
    match double_quote_4(lexbuf) {
      _ => {
        let i = lexbuf.get_read();
        lexbuf.reset_current();
        lexbuf.set_read(0);
        if lexbuf.pos_e_cnum <= lexbuf.pos_b_cnum { Err("Reached End of File without any corrresponding rule") }
		else if i == 61 { 
        lexbuf.newline();
        let mut s1 = String::from("\n");
        match double_quote(lexbuf) {
            Ok(parser::Token::WORD(s)) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        Ok(parser::Token::WORD(s1))
     }
		else if i == 63 { let t1 = lexbuf.get_token();  Ok(parser::Token::WORD(String::new()))  }
		else if i == 65 { let t2 = lexbuf.get_token(); 
        let mut s1 = String::from("\"");
        match double_quote(lexbuf) {
            Ok(parser::Token::WORD(s)) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        Ok(parser::Token::WORD(s1))
     }
		else if i == 68 { let c = lexbuf.get_token(); 
        let mut s1 = String::from(c.chars().nth(1).unwrap());
        match double_quote(lexbuf) {
            Ok(parser::Token::WORD(s)) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        Ok(parser::Token::WORD(s1))
     }
		else if i == 71 { let s = lexbuf.get_token(); 
        let mut s1 = String::from(s);
        match double_quote(lexbuf) {
            Ok(parser::Token::WORD(s)) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        Ok(parser::Token::WORD(s1))
      
     }
		else { Err("Undefined rule, should not happen, please report this")
		}
		}
	}
	}
}
fn double_quote_0(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(61);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_1(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(63);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_2(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(68);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_3(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(71);
	if let Ok(chr) = lexbuf.next_char() {
	if (93 <= chr && chr <= 255)||(35 <= chr && chr <= 91)||(11 <= chr && chr <= 33)||chr <= 9 { double_quote_3(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_4(lexbuf : &mut Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if chr == 10 { double_quote_0(lexbuf.incr_pos()) } else
	if chr == 34 { double_quote_1(lexbuf.incr_pos()) } else
	if (93 <= chr && chr <= 255)||(35 <= chr && chr <= 91)||(11 <= chr && chr <= 33)||chr <= 9 { double_quote_3(lexbuf.incr_pos()) } else
	if chr == 92 { double_quote_5(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_5(lexbuf : &mut Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if chr <= 255 { double_quote_2(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
pub fn comment (lexbuf: &mut Lexbuf) -> Result<parser::Token, &'static str> { 
	if lexbuf.finished() {
		 Err("end of string not handled") 
	} else {
    lexbuf.new_extremity();
    match comment_2(lexbuf) {
      _ => {
        let i = lexbuf.get_read();
        lexbuf.reset_current();
        lexbuf.set_read(0);
        if lexbuf.pos_e_cnum <= lexbuf.pos_b_cnum { Err("Reached End of File without any corrresponding rule") }
		else if i == 73 {  lexbuf.newline(); comment(lexbuf)  }
		else if i == 75 {  comment(lexbuf)  }
		else { Err("Undefined rule, should not happen, please report this")
		}
		}
	}
	}
}
fn comment_0(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(73);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn comment_1(lexbuf : &mut Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(75);
	if let Ok(chr) = lexbuf.next_char() {
	{ Err(()) }
	} else {Err(())}
}
fn comment_2(lexbuf : &mut Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if chr == 10 { comment_0(lexbuf.incr_pos()) } else
	if (11 <= chr && chr <= 255)||chr <= 9 { comment_1(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}



