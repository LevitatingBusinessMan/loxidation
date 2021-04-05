use crate::chunk::{Chunk, op_codes::*, value::{Value,number}};
use crate::scanner::{Scanner, tokens::{*}};

#[macro_use]
mod rules;

use rules::*;

struct Compiler {
	previous: Token,
	current: Token,
	scanner: Scanner,
	success: bool,
	panic: bool,
	chunk: Chunk
}

pub fn compile(source: String) -> Result<Chunk, ()> {
	let scanner = Scanner::new(source);

	let placeholder_token = Token {
		ttype: TokenType::EOF,
		start: 0,
		length: 0,
		line: 0
	};

	let mut compiler = Compiler {
		scanner,
		current: placeholder_token,
		previous: placeholder_token,
		chunk: Chunk::new(),
		panic: false,
		success: true
	};


	return match compiler.start() {
		Ok(()) => Ok(compiler.chunk),
		Err(()) => Err(())
	}
}

impl Compiler {
	pub fn start(&mut self) -> Result<(), ()> {
		
		self.advance();

		while self.current.ttype != TokenType::EOF {
			self.decleration();
		}

		self.push_byte(RETURN);

		if !self.success {
			return Err(());
		} else {
			return Ok(());
		}

	}

	fn expression(&mut self) {
		//Lowest
		self.parse_precedence(Precedence::Assignment);
	}

	fn decleration(&mut self) {
		self.statement()
	}

	fn statement(&mut self) {
		match self.current.ttype {
			TokenType::PRINT => self.print_statement(),
			_ => unreachable!()
		}
	}

	fn print_statement(&mut self) {
		self.advance();
		self.expression();
		self.consume(TokenType::SEMICOLON, "expected ';' after expression");
		self.push_byte(PRINT);
	}

	fn parse_precedence(&mut self, prec: impl Into<u32>) {
		let prec = prec.into();
		self.advance();
		let rule = get_rule(self.previous.ttype);
		
		if let Some(prefix) = rule.prefix {
				prefix(self);

				while prec <= get_rule(self.current.ttype).precedence as u32 {
					self.advance();
					let new_rule = get_rule(self.previous.ttype);
					if let Some(infix) = new_rule.infix {
						infix(self);
					} else {
						unreachable!();
					}
				}

		} else {
			self.error_at(self.previous, "expected expression")
		}
	}

	fn unary(&mut self) {
		let op_type = self.previous.ttype;
		
		self.parse_precedence(Precedence::Unary);

		match op_type {
			TokenType::MINUS => self.push_byte(NEGATE),
			TokenType::BANG => self.push_byte(NOT),
			_ => unreachable!()
		}
	}

	fn grouping(&mut self) {
		self.expression();
		self.consume(TokenType::RIGHT_PAREN, "expected ')' after expression");
	}

	fn binary(&mut self) {
		//First operand is compiled

		let op_type = self.previous.ttype;

		let rule = get_rule(op_type);

		//Push the other operand
		self.parse_precedence(rule.precedence as u32+1);

		//Operator time
		match op_type {
			TokenType::PLUS => self.push_byte(ADD),
			TokenType::MINUS => self.push_byte(SUBTRACT),
			TokenType::ASTERISK => self.push_byte(MULTIPLY),
			TokenType::SLASH => self.push_byte(DIVIDE),
			TokenType::EQUAL_EQUAL => self.push_byte(EQUAL),
			TokenType::BANG_EQUAL => self.push_bytes(&[EQUAL, NOT]),
			TokenType::GREATER => self.push_byte(GREATER),
			TokenType::GREATER_EQUAL => self.push_bytes(&[LESS,NOT]),
			TokenType::LESS => self.push_byte(LESS),
			TokenType::LESS_EQUAL => self.push_bytes(&[GREATER, NOT]),
			_ => unreachable!()
		}
		
	}

	fn literal(&mut self) {
		match self.previous.ttype {
			TokenType::NIL => self.push_byte(NIL),
			TokenType::TRUE => self.push_byte(TRUE),
			TokenType::FALSE => self.push_byte(FALSE),
			_ => unreachable!()
		}
	}

	fn string(&mut self) {
		let lexeme = self.lexeme(self.previous);
		let value = Value::from(lexeme[1..lexeme.len() - 1].to_owned());
		self.push_constant(value);
	}

	fn number(&mut self) {
		//I bet it's safe to just assume this will parse to an int, right?
		let value = Value::from(self.lexeme(self.previous).parse::<number>().unwrap());
		self.push_constant(value);
	}

	fn push_constant(&mut self, value: Value) {
		if self.chunk.constants.len() >= 256^std::mem::size_of::<OpCode>() {
			self.error_at(self.previous, "reached constant limit");
		}
		let index = self.chunk.push_constant(value) as OpCode;
		self.push_bytes(&[CONSTANT, index]);
	}

	fn lexeme(&self, token: Token) -> &str {
		&self.scanner.source[token.start..token.start+(token.length as usize)]
	}

	fn advance(&mut self) {
		self.previous = self.current;
		loop {
			match self.scanner.scan_token() {
				TokenResult::TOKEN(token) => {
					self.current = token;
					break;
				}, TokenResult::ERROR(error) => {
					self.error(error)
				}
			}

		}
	}

	fn push_byte(&mut self, op: OpCode) {
		self.chunk.push_op(op, self.previous.line);
	}

	fn push_bytes(&mut self, ops: &[OpCode]) {
		for op in ops {
			self.push_byte(*op);
		}
	}

	fn consume(&mut self, ttype: TokenType, errmsg: impl AsRef<str>) {
		if self.current.ttype == ttype {
			self.advance();
		} else {
			self.error_at(self.current, errmsg);
		}
	}

	fn error_at(&mut self, token: Token, msg: impl AsRef<str>) {
		let msg = msg.as_ref();
		if token.ttype == TokenType::EOF {
			self.print_error(format!("[ERR {}]: at EOF {}", token.line, msg));
		} else {
			let lexeme = self.lexeme(token).to_owned();
			self.print_error(format!("[ERR {}]: at '{}' {}", token.line, lexeme, msg));
		}
	}

	fn error(&mut self, error: TokenError) {
		self.print_error(format!("[ERR {}]: {}", error.line, error.message));
	}

	fn print_error(&mut self, msg: String) {
		if self.panic {return;}
		println!("{}",msg);
		self.success = false;
		self.panic = true;
	}
}
