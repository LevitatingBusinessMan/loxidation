pub mod tokens;

use self::tokens::*;
use std::char;

pub struct Scanner {
	start: usize,
	current: usize,
	line: u32,
	pub source: String
}

impl Scanner {
	pub fn new(source: String) -> Scanner {
		Scanner {
			start: 0,
			current: 0,
			line: 1,
			source
		}
	}

	/// Not used
	#[allow(dead_code)]
	pub fn scan_all(&mut self) -> Vec<TokenResult> {
		let mut tokens = Vec::<TokenResult>::new();
		loop {
			let mut token = self.scan_token();

			//Check if it's EOF, stopping the loop
			//But also make sure to move the token back as an enum
			if let TokenResult::TOKEN(t) = token {
				if t.ttype == TokenType::EOF {
					tokens.push(TokenResult::TOKEN(t));
					break;
				}
				token = TokenResult::TOKEN(t);
			}

			tokens.push(token);
		}
		return tokens;
	}

	pub fn scan_token(&mut self) -> TokenResult {
		self.start = self.current;

		macro_rules! token {($type:ident) => {TokenResult::TOKEN(self.token(TokenType::$type))};}
		macro_rules! error {($msg:expr) => {TokenResult::ERROR(self.error_token($msg.to_owned()))};}

		if self.at_end() {return token!(EOF)};

		let mut character = self.advance();

		// Glorious whitespace removal loop
		loop {
			if character == '/' && self.peek() == Some('/') {
				if self.consume_till('\n') {
					if self.peek() == None {
						return token!(EOF);
					}
					character = self.advance();
					self.line += 1;
				} else {
					return token!(EOF);
				}
				continue;
			}
			if character == '/' && self.peek() == Some('*') {
				self.advance();
				loop {
					if self.consume_till('*') {
						if self.peek() == Some('/') && !self.at_end() {
							self.advance();
							character = self.advance();
							break;
						} else {
							return token!(EOF);
						}
					} else {
						return token!(EOF);
					}
				}
				continue;
			}

			//All of this is a bit of a mess I know but it works flawlessly
			self.start = self.current-1;
			if !character.is_whitespace() && (character != '/' || (self.peek() != Some('*') && self.peek() != Some('/'))){break;}
			if character == '\n' {self.line += 1;}

			if self.at_end() {
				return token!(EOF)
			}

			character = self.advance();
		}

		// Glorious digit loop
		if character.is_digit(10) || 
			// floats starting with a dot
			character == '.' && self.peek().unwrap_or(' ').is_digit(10)
			{	
			let mut seen_dot = character == '.';
			loop {
				match self.peek() {
					Some(next) => {
						if !next.is_digit(10) {
							if seen_dot || next != '.' {
								return token!(NUMBER);
							} else if !seen_dot && next == '.' {
								seen_dot = true;
							}
						}
					},
					None => return token!(NUMBER)
				}
				self.advance();
			}
		}

		// Glorious identifier loop
		if character.is_alphabetic() || character == '_' {
			loop {
				if self.peek() == None || !self.peek().unwrap().is_alphabetic() && self.peek().unwrap() != '_' {
					//Yes bob I know trie's are faster
					//But guess what, this isn't C and you aren't my dad
					let string = &self.source[self.start..self.current];

					return match string {
						"and" => token!(AND),
						"class" => token!(CLASS),
						"else" => token!(ELSE),
						"false" => token!(FALSE),
						"for" => token!(FOR),
						"fun" => token!(FUN),
						"if" => token!(IF),
						"nil" => token!(NIL),
						"or" => token!(OR),
						"print" => token!(PRINT),
						"return" => token!(RETURN),
						"super" => token!(SUPER),
						"this" => token!(THIS),
						"true" => token!(TRUE),
						"var" | "let" => token!(VAR),
						"const" => token!(CONST),
						"while" => token!(WHILE),
						"goto" => token!(GOTO),
						"label" => token!(LABEL),
						"continue" => token!(CONTINUE),
						_ => token!(IDENTIFIER)
					}
				}
				self.advance();
			}
		}

		let character = character;

		return match character {
			'(' => token!(LEFT_PAREN),
			')' => token!(RIGHT_PAREN),
			'{' => token!(LEFT_BRACE),
			'}' => token!(RIGHT_BRACE),
			';' => token!(SEMICOLON),
			',' => token!(COMMA),
			'.' => token!(DOT),
			'-' => token!(MINUS),
			'+' => token!(PLUS),
			'/' => token!(SLASH),
			'*' => token!(ASTERISK),
			'?' => token!(QUESTION),
			':' => token!(COLON),
			'|' => if self.peek() == Some('|') {self.advance(); token!(OR)} else {token!(PIPE)},
			'&' => if self.peek() == Some('&') {self.advance(); token!(AND)} else {token!(AMPERSAND)},
			'!' => if self.peek() == Some('=') {self.advance(); token!(BANG_EQUAL)} else {token!(BANG)},
			'=' => if self.peek() == Some('=') {self.advance(); token!(EQUAL_EQUAL)} else {token!(EQUAL)},
			'<' => if self.peek() == Some('=') {self.advance(); token!(LESS_EQUAL)} else {token!(LESS)},
			'>' => if self.peek() == Some('=') {self.advance(); token!(GREATER_EQUAL)} else {token!(GREATER)},
			'"' => if self.consume_till('"') {token!(STRING)} else {error!("non-terminated string")},
			'\'' => if self.consume_till('\'') {
				if self.current - self.start == 3 {
					token!(CHAR)
				} else {
					error!("invalid char")
				}
			} else {error!("non-terminated char")}
			_ => error!(format!("unknown token '{}'", character)),
		};

	}

	// Returns true if it succesfully found the character
	// Or false if it reached the end
	fn consume_till(&mut self, c: char) -> bool {
		while self.peek() != Some(c) || self.peek() == None {
			if self.peek() == None {
				return false;
			}
			if self.advance() == '\n' {
				self.line += 1
			}
		}
		self.advance();
		return true;
	}

	fn at_end(&self) -> bool {
		self.source.len() == self.current
	}

	fn token(&self, ttype: TokenType) -> Token {
		Token {
			ttype,
			start: self.start,
			length: (self.current - self.start) as u32,
			line: self.line
		}
	}

	fn error_token(&self, msg: String) -> TokenError {
		TokenError {
			message: msg,
			line: self.line
		}
	}

	fn advance(&mut self) -> char {
		self.current += 1;
		self.source.as_bytes()[self.current-1] as char
	}

	fn peek(&mut self) -> Option<char> {
		if self.at_end() {return None}
		Some(self.source.as_bytes()[self.current] as char)
	}

}
