
use alloc::string::String;

pub struct Lexbuf {
  pos_b_lnum : usize,
  pos_b_cnum : usize,
  pos_e_cnum : usize,
  current : usize,
  read : usize,
  text : &'static [u8],
  len : usize,
}

impl Lexbuf {
  fn new_line(&mut self) {
    self.pos_b_lnum = self.pos_b_lnum + 1;
  }

  fn incr_pos(self) -> Self {
    self.current = self.current + 1;
    self
  }

  pub fn new(text : &'static str) -> Self {
    Lexbuf {
      pos_b_lnum : 1,
      pos_b_cnum : 0,
      pos_e_cnum : 0,
      current : 0,
      read : 0,
      text : text.as_bytes(),
      len : text.len(),
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
      Ok(self.text[self.current] as u8)
    }
  }


  fn finished(&mut self) -> bool {
    self.len == self.current
  }

  fn new_extremity(&mut self) {
    self.pos_b_cnum = self.pos_e_cnum
  }

  fn get_token(&self) -> &[u8] {
    &self.text[self.pos_b_cnum..self.pos_e_cnum+1]
  }

}


    use super::command
    use super::parser

    enum LexicalErr {
        UnclosedComment,
    };

    pub struct LexicalError(pub LexicalErr);

    fun keyword_of_string(str: &str) -> Option<parser::Token> {
        unsafe {
            super::first_word = false
        };

        match str with {
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


fn token (mut lexbuf: &Lexbuf) -> Result<parser::Token, &'static str> { 
	if lexbuf.finished() {
		 Err(()) 
	} else {
    lexbuf.new_extremity();
    match token_18(lexbuf) {
      _ => {
        let i = lexbuf.get_read();
        lexbuf.reset_current();
        lexbuf.set_read(0);
        if lexbuf.pos_e_cnum <= lexbuf.pos_b_cnum { Err("Reached End of File without any corrresponding rule") }
		else if i == 15 {  comment(lexbuf); token(lexbuf)  }
		else if i == 17 {  newline(lexbuf); parser::Token::NEWLINE  }
		else if i == 20 {  token(lexbuf) }
		else if i == 22 {  parser::Token::LESS  }
		else if i == 24 {  parser::Token::GREATER  }
		else if i == 27 {  parser::Token::GREATER_GREATER  }
		else if i == 29 {  parser::Token::AND  }
		else if i == 32 {  parser::Token::AND_AND  }
		else if i == 35 {  parser::Token::OR_OR  }
		else if i == 37 {  parser::Token::BAR  }
		else if i == 39 {  parser::Token::SEMI  }
		else if i == 46 { let s = lexbuf.get_token(); 
        match unsafe { Msh_misc.first_word }, keyword_of_string(s) {
        true, Some kw => kw,
        _ =>
            let mut lb = Lexbuf::new(s);
            word(lb)
            }
         }
		else if i == 48 { 
        double_quote(String::new(), lexbuf)
       }
		else if i == 50 {  parser::Token::EOF  }
		else if i == 52 {  panic!("error") }
		else { Err("Undefined rule, should not happen, please report this")
		}
	}
}
fn token_0(mut lexbuf : &Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { token_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_1(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(15);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 124||chr == 62||(59 <= chr && chr <= 60)||chr == 38||chr == 34||chr == 32||(9 <= chr && chr <= 10) { token_0(lexbuf.incr_pos()) } else
	if chr == 92 { token_15(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||(0 <= chr && chr <= 8) { token_16(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_2(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(17);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { token_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_3(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(20);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { token_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_4(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(22);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { token_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_5(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(24);
	if let Ok(chr) = lexbuf.next_char() {
	if (63 <= chr && chr <= 255)||(0 <= chr && chr <= 61) { token_0(lexbuf.incr_pos()) } else
	if chr == 62 { token_6(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_6(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(27);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { token_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_7(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(29);
	if let Ok(chr) = lexbuf.next_char() {
	if (39 <= chr && chr <= 255)||(0 <= chr && chr <= 37) { token_0(lexbuf.incr_pos()) } else
	if chr == 38 { token_8(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_8(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(32);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { token_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_9(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(35);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { token_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_10(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(37);
	if let Ok(chr) = lexbuf.next_char() {
	if (125 <= chr && chr <= 255)||(0 <= chr && chr <= 123) { token_0(lexbuf.incr_pos()) } else
	if chr == 124 { token_9(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_11(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(39);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { token_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_12(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(46);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 124||chr == 62||(59 <= chr && chr <= 60)||chr == 38||chr == 34||chr == 32||(9 <= chr && chr <= 10) { token_0(lexbuf.incr_pos()) } else
	if chr == 92 { token_15(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||(0 <= chr && chr <= 8) { token_16(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_13(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(46);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 124||chr == 62||(59 <= chr && chr <= 60)||chr == 38||chr == 34||chr == 32||(9 <= chr && chr <= 10) { token_0(lexbuf.incr_pos()) } else
	if chr == 92 { token_15(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||(0 <= chr && chr <= 8) { token_16(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_14(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(46);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 124||chr == 62||(59 <= chr && chr <= 60)||chr == 38||chr == 34||chr == 32||(9 <= chr && chr <= 10) { token_0(lexbuf.incr_pos()) } else
	if chr == 92 { token_15(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||(0 <= chr && chr <= 8) { token_16(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_15(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(46);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 124||chr == 62||(59 <= chr && chr <= 60)||chr == 38||chr == 34||chr == 32||(9 <= chr && chr <= 10) { token_0(lexbuf.incr_pos()) } else
	if chr == 92 { token_15(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||(0 <= chr && chr <= 8) { token_16(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_16(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(46);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 124||chr == 62||(59 <= chr && chr <= 60)||chr == 38||chr == 34||chr == 32||(9 <= chr && chr <= 10) { token_0(lexbuf.incr_pos()) } else
	if chr == 92 { token_15(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||(0 <= chr && chr <= 8) { token_16(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_17(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(48);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { token_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn token_18(mut lexbuf : &Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if chr == 35 { token_1(lexbuf.incr_pos()) } else
	if chr == 10 { token_2(lexbuf.incr_pos()) } else
	if chr == 32||chr == 9 { token_3(lexbuf.incr_pos()) } else
	if chr == 60 { token_4(lexbuf.incr_pos()) } else
	if chr == 62 { token_5(lexbuf.incr_pos()) } else
	if chr == 38 { token_7(lexbuf.incr_pos()) } else
	if chr == 124 { token_10(lexbuf.incr_pos()) } else
	if chr == 59 { token_11(lexbuf.incr_pos()) } else
	if chr == 0 { token_12(lexbuf.incr_pos()) } else
	if chr == 92 { token_13(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(36 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||(1 <= chr && chr <= 8) { token_14(lexbuf.incr_pos()) } else
	if chr == 34 { token_17(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn word (mut lexbuf: &Lexbuf) -> Result<parser::Token, &'static str> { 
	if lexbuf.finished() {
		 Err(()) 
	} else {
    lexbuf.new_extremity();
    match word_7(lexbuf) {
      _ => {
        let i = lexbuf.get_read();
        lexbuf.reset_current();
        lexbuf.set_read(0);
        if lexbuf.pos_e_cnum <= lexbuf.pos_b_cnum { Err("Reached End of File without any corrresponding rule") }
		else if i == 54 {  panic!("Error")  }
		else if i == 56 {  
        let mut s1 = match double_quote(lexbuf) {
            parser::Token::Word(s) => s,
            _ => panic!("should not happen"),
        }
        match word(lexbuf) {
            parser::Token::Word(s) => s1.push_str(s),
            _ => panic!("should not happen"),
        }
        parser::Token::Word(s1)
       }
		else if i == 59 { let c = lexbuf.get_token();  let s1 = String::from(c[1]);
        match word(lexbuf) {
            parser::Token::Word(s) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        parser::Token::Word(s1)
        
     }
		else if i == 61 {  parser::Token::Word(String::new())  }
		else if i == 63 { let c = lexbuf.get_token(); 
        let s1 = String::from(c[1]);
        match word(lexbuf) {
            parser::Token::Word(s) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        parser::Token::Word(s1)
       }
		else { Err("Undefined rule, should not happen, please report this")
		}
	}
}
fn word_0(mut lexbuf : &Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { word_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn word_1(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(54);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { word_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn word_2(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(56);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { word_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn word_3(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(59);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { word_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn word_4(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(61);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { word_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn word_5(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(63);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { word_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn word_6(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(63);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { word_3(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn word_7(mut lexbuf : &Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if chr == 124||chr == 62||(59 <= chr && chr <= 60)||chr == 38||chr == 32||(9 <= chr && chr <= 10) { word_1(lexbuf.incr_pos()) } else
	if chr == 34 { word_2(lexbuf.incr_pos()) } else
	if chr == 0 { word_4(lexbuf.incr_pos()) } else
	if (125 <= chr && chr <= 255)||(93 <= chr && chr <= 123)||(63 <= chr && chr <= 91)||chr == 61||(39 <= chr && chr <= 58)||(35 <= chr && chr <= 37)||chr == 33||(11 <= chr && chr <= 31)||(1 <= chr && chr <= 8) { word_5(lexbuf.incr_pos()) } else
	if chr == 92 { word_6(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote (mut lexbuf: &Lexbuf) -> Result<parser::Token, &'static str> { 
	if lexbuf.finished() {
		 Err(()) 
	} else {
    lexbuf.new_extremity();
    match double_quote_6(lexbuf) {
      _ => {
        let i = lexbuf.get_read();
        lexbuf.reset_current();
        lexbuf.set_read(0);
        if lexbuf.pos_e_cnum <= lexbuf.pos_b_cnum { Err("Reached End of File without any corrresponding rule") }
		else if i == 65 { 
        newline lexbuf;
        let s1 = String::from("\n");
        match double_quote(lexbuf) {
            parser::Token::Word(s) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        parser::Token::Word(s1)
     }
		else if i == 67 { let t1 = lexbuf.get_token();  parser::Token::Word(String::new())  }
		else if i == 69 { let t2 = lexbuf.get_token(); 
        let s1 = String::from("\"");
        match double_quote(lexbuf) {
            parser::Token::Word(s) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        parser::Token::Word(s1)
     }
		else if i == 72 { let c = lexbuf.get_token(); 
        let s1 = String::from(c);
        match double_quote(lexbuf) {
            parser::Token::Word(s) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        parser::Token::Word(s1)
     }
		else if i == 75 { let s = lexbuf.get_token(); 
        let s1 = String::from(s);
        match double_quote(lexbuf) {
            parser::Token::Word(s) => s1.push_str(&s),
            _ => panic!("should not happen"),
        }
        parser::Token::Word(s1)
      
     }
		else if i == 77 {  panic!("Unclosed String")  }
		else { Err("Undefined rule, should not happen, please report this")
		}
	}
}
fn double_quote_0(mut lexbuf : &Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { double_quote_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_1(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(65);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { double_quote_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_2(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(67);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { double_quote_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_3(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(72);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { double_quote_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_4(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(75);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 92||chr == 34||chr == 10 { double_quote_0(lexbuf.incr_pos()) } else
	if (93 <= chr && chr <= 255)||(35 <= chr && chr <= 91)||(11 <= chr && chr <= 33)||(0 <= chr && chr <= 9) { double_quote_5(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_5(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(75);
	if let Ok(chr) = lexbuf.next_char() {
	if chr == 92||chr == 34||chr == 10 { double_quote_0(lexbuf.incr_pos()) } else
	if (93 <= chr && chr <= 255)||(35 <= chr && chr <= 91)||(11 <= chr && chr <= 33)||(0 <= chr && chr <= 9) { double_quote_5(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_6(mut lexbuf : &Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if chr == 10 { double_quote_1(lexbuf.incr_pos()) } else
	if chr == 34 { double_quote_2(lexbuf.incr_pos()) } else
	if chr == 0 { double_quote_4(lexbuf.incr_pos()) } else
	if (93 <= chr && chr <= 255)||(35 <= chr && chr <= 91)||(11 <= chr && chr <= 33)||(1 <= chr && chr <= 9) { double_quote_5(lexbuf.incr_pos()) } else
	if chr == 92 { double_quote_7(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn double_quote_7(mut lexbuf : &Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { double_quote_3(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn comment (mut lexbuf: &Lexbuf) -> Result<parser::Token, &'static str> { 
	if lexbuf.finished() {
		 Err(()) 
	} else {
    lexbuf.new_extremity();
    match comment_3(lexbuf) {
      _ => {
        let i = lexbuf.get_read();
        lexbuf.reset_current();
        lexbuf.set_read(0);
        if lexbuf.pos_e_cnum <= lexbuf.pos_b_cnum { Err("Reached End of File without any corrresponding rule") }
		else if i == 79 {  newline(lexbuf); comment(lexbuf)  }
		else if i == 81 {  comment(lexbuf)  }
		else { Err("Undefined rule, should not happen, please report this")
		}
	}
}
fn comment_0(mut lexbuf : &Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { comment_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn comment_1(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(79);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { comment_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn comment_2(mut lexbuf : &Lexbuf) -> Result<(),()> { 
	lexbuf.set_read(81);
	if let Ok(chr) = lexbuf.next_char() {
	if (0 <= chr && chr <= 255) { comment_0(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}
fn comment_3(mut lexbuf : &Lexbuf) -> Result<(),()> { if let Ok(chr) = lexbuf.next_char() {
	if chr == 10 { comment_1(lexbuf.incr_pos()) } else
	if (11 <= chr && chr <= 255)||(0 <= chr && chr <= 9) { comment_2(lexbuf.incr_pos()) } else
	{ Err(()) }
	} else {Err(())}
}



