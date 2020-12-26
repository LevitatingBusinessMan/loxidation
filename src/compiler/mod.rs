use crate::chunk::{Chunk, op_codes::*, value::Value};
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
	let scanner = Scanner::new(source.clone());

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

		self.expression();

		self.consume(TokenType::EOF, "missing EOF at end");

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
		
		self.expression();

		match op_type {
			TokenType::MINUS => self.push_byte(NEGATE),
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
			TokenType::MINUS => self.push_byte(NEGATE),
			TokenType::ASTERISK => self.push_byte(MULTIPLY),
			TokenType::SLASH => self.push_byte(DIVIDE),
			_ => unreachable!()
		}
		
	}

	fn number(&mut self) {

		//Gotta use this result
		let value = self.lexeme(self.previous).parse::<Value>();
		match value {
			Ok(value) => {
				self.push_constant(value);
			},
			Err(_) => {
				self.error_at(self.previous, "lexeme doesn't convert to Lox value");
			}
		}
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