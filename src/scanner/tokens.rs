#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
	// Single-character tokens.
	LEFT_PAREN, RIGHT_PAREN,
	LEFT_BRACE, RIGHT_BRACE,
	COMMA, DOT, MINUS, PLUS,
	SEMICOLON, SLASH, ASTERISK,
  
	// One or two character tokens.
	BANG, BANG_EQUAL,
	EQUAL, EQUAL_EQUAL,
	GREATER, GREATER_EQUAL,
	LESS, LESS_EQUAL,
  
	// Literals.
	IDENTIFIER, STRING, NUMBER,
  
	// Keywords.
	AND, CLASS, ELSE, FALSE,
	FOR, FUN, IF, NIL, OR,
	PRINT, RETURN, SUPER, THIS,
	TRUE, VAR, WHILE,
  
	EOF
}

#[derive(Debug)]
pub struct Token { 
	pub ttype: TokenType,
	pub start: usize,
	pub length: u32,
	pub line: u32
}

#[derive(Debug)]
pub struct TokenError {
	pub message: String,
	pub line: u32
}

#[derive(Debug)]
pub enum TokenResult {
	TOKEN(Token),
	ERROR(TokenError)
}
