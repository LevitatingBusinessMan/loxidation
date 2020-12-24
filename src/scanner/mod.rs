pub mod tokens;

use tokens::TokenType;

pub struct Scanner {
	start: usize,
	current: usize,
	line: u32,
	source: String
}

#[derive(Debug)]
pub struct Token { 
	ttype: TokenType,
	start: usize,
	length: u32,
	line: u32
}

#[derive(Debug)]
pub struct TokenError {
	message: String,
	line: u32
}

#[derive(Debug)]
pub enum TokenResult {
	TOKEN(Token),
	ERROR(TokenError)
}

impl Scanner {
	pub fn new(source: String) -> Scanner {
		Scanner {
			start: 0,
			current: 0,
			line: 0,
			source
		}
	}

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

		if self.at_end() {return token!(EOF)};

		let mut character = self.advance();

		//Glorious whitespace removal loop
		loop {
			if character == '/' && self.peek() == Some('=') {
				while self.peek() != Some('=') {
					self.advance();
					if self.at_end() {
						return token!(EOF)
					}
				}
			}
			if !character.is_whitespace() {break;}
			if character == '\n' {self.line += 1;}

			if self.at_end() {
				return token!(EOF)
			}
			character = self.advance();
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
			'!' => if self.peek() == Some('=') {self.advance(); token!(BANG_EQUAL)} else {token!(BANG)},
			'=' => if self.peek() == Some('=') {self.advance(); token!(EQUAL_EQUAL)} else {token!(EQUAL)},
			'<' => if self.peek() == Some('=') {self.advance(); token!(LESS_EQUAL)} else {token!(LESS)},
			'>' => if self.peek() == Some('=') {self.advance(); token!(GREATER_EQUAL)} else {token!(GREATER)},
			_ => TokenResult::ERROR(self.error_token("Unknown token".to_owned()))
		};

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
		if self.source.len() == self.current+1 {return None}
		Some(self.source.as_bytes()[self.current+1] as char)
	}

}
