use crate::vm::chunk::Chunk;
use crate::vm::value::Value;
use crate::vm::op_codes::*;
use crate::scanner::{Scanner, tokens::{*}};
use crate::vm::value::number;

#[macro_use]
mod rules;

use self::rules::*;

#[derive(PartialEq, Debug)]
struct Local {
	/// Identifier token where the local was initialized
	identifier: Token,

	/// The depth the local was declared at
	depth: u32,

	/// If the local has been initialized
	/// The book doesn't use this but instead
	/// sets the depth to -1
	initialized: bool,

	/// If the local is a constant
	constant: bool,
}

struct Global {
	identifier: Token,
	constant: bool,
}

/// Holds the state of the compiler
struct Compiler {
	previous: Token,
	current: Token,
	scanner: Scanner,
	success: bool,
	panic: bool,
	can_assign: bool,
	chunk: Chunk,
	
	/// Locals in scope
	locals: Vec<Local>,
	globals: Vec<Global>,

	/// Scope depth
	scope: u32,
}

impl Compiler {
	pub fn new(scanner: Scanner) -> Compiler {
		let placeholder_token = Token {
			ttype: TokenType::EOF,
			start: 0,
			length: 0,
			line: 0
		};
	
		Compiler {
			scanner,
			current: placeholder_token,
			previous: placeholder_token,
			chunk: Chunk::new(),
			panic: false,
			success: true,
			can_assign: false,
			locals: vec![],
			globals: vec![],
			scope: 0,
		}
	}
}

pub fn compile(source: String) -> Result<Chunk, ()> {
	let mut compiler = Compiler::new(Scanner::new(source));

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
			if self.panic {
				self.synchronize();
			}
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
		match self.current.ttype {
			TokenType::VAR => self.var_decleration(false),
			TokenType::CONST => {
				self.var_decleration(true);
			}
			_ => self.statement()
		}
	}

	fn var_decleration(&mut self, constant: bool) {
		//Advance over the var token
		self.advance();
		
		let global_index = self.parse_variable("expected variable name", constant);
		
		if self.current.ttype == TokenType::EQUAL {
			self.advance();
			self.expression();
		} else {
			self.push_byte(NIL);
		}

		self.consume(TokenType::SEMICOLON, "expected ';' after variable decleration");

		if global_index.is_some() {
			self.define_global(global_index.unwrap())
		} else {
			// After we have parsed the expression of a local
			// we can say it's initialized
			self.locals.last_mut().unwrap().initialized = true;
		};
	}

	/// In case of a global, consumes the identifier and saves it as a string in the constants
	/// then returns the index.
	/// This will handle globals, locals functions, classes? and parameters
	fn parse_variable(&mut self, errormsg: &str, constant: bool) -> Option<usize> {
		self.consume(TokenType::IDENTIFIER, errormsg);
		if self.scope > 0 {
			self.declare_variable(constant);
			return None;
		}
		Some(self.set_global(self.previous, constant))
	}

	/// Save a local
	fn declare_variable(&mut self, constant: bool) {		
		// Detect a double variable decleration
		let mut error: Option<Token> = None;
		for local in &self.locals {
			if local.identifier == self.previous {
				error = Some(self.previous);
				break;
			}
		}
		if let Some(error) = error {
			self.error_at(error, "A variable with that identifier has already been declared in this scope");
		}

		let local = Local {
			identifier: self.previous,
			depth: self.scope,
			initialized: false,
			constant,
		};
		self.locals.push(local);
	}

	/// Saves a global and returns the index
	fn set_global(&mut self, identifier: Token, constant: bool) -> usize {
		let lexeme = self.lexeme(identifier).to_string();
		self.globals.iter().position(|global| self.lexeme(global.identifier) == lexeme).unwrap_or_else(|| {
			self.globals.push(Global {
				identifier,
				constant,
			});
			self.globals.len()-1
		})
	}

	fn get_global(&mut self, identifier: Token, complain_const: bool) -> Option<usize> {
		let lexeme = self.lexeme(identifier).to_string();
		let mut index: Option<usize> = None;
		let mut error: Option<&str> = None;
		for (i, global) in self.globals.iter().enumerate() {
			if self.lexeme(global.identifier) == lexeme {
				if global.constant && complain_const {
					error = Some("Can't redefine constant");
				}
				index = Some(i);
			}
		}
		if let Some(error) = error {
			self.error_at(identifier, error);
		}
		return index;
	}

	/// Find a local by token, return the index
	fn resolve_local(&mut self, identifier: Token, complain_const: bool) -> Option<usize> {
		let mut index: Option<usize> = None;
		let mut error: Option<&str> = None;
		for (i, local) in self.locals.iter().rev().enumerate() {
			if self.lexeme(local.identifier) == self.lexeme(identifier) {

				// This means we are still inside this locals initializer
				if !local.initialized {
					error = Some("Can't read local variable in it's own initializer.");
				}

				// This local is a constant and should not be redefined
				if local.constant && complain_const {
					error = Some("Can't redefine constant");
				}

				// The iterator is reversed to check most recent locals first,
				// but this means we have to convert the index as well
				index = Some(self.locals.len()-i-1);
				break;
			}
		}
		if let Some(error) = error {
			self.error_at(identifier, error);
		}
		return index;
	}

	fn define_global(&mut self, global_index: usize) {
		self.push_byte(DEFGLOBAL);
		self.push_byte(global_index as u8);
	}

	fn statement(&mut self) {
		match self.current.ttype {
			TokenType::PRINT => self.print_statement(),
			TokenType::LEFT_BRACE => {
				self.begin_scope();
				self.block_statement();
				self.end_scope();
			},
			_ => self.expression_statement()
		}
	}

	fn block_statement(&mut self) {
		self.advance();
		while self.current.ttype != TokenType::RIGHT_BRACE && self.current.ttype != TokenType::EOF {
			self.decleration();
		}
		self.consume(TokenType::RIGHT_BRACE, "expected '}' after block");
	}

	fn begin_scope(&mut self) {
		self.scope += 1;
	}

	fn end_scope(&mut self) {
		self.scope -= 1;
		let len_before = self.locals.len();
		let scope = self.scope;
		self.locals.retain(|local| local.depth <= scope);
		
		// Create a pop for each removed
		for _ in 0..(len_before - self.locals.len()) {
			self.push_byte(POP);
		}
	}

	fn expression_statement(&mut self) {
		self.expression();
		self.consume(TokenType::SEMICOLON, "expected ';' after expression");
		self.push_byte(POP);
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
				let can_assign = prec <= Precedence::Assignment as u32;
				self.can_assign = can_assign;
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

				if can_assign && self.current.ttype == TokenType::EQUAL {
					self.error_at(self.previous, "invalid assignment target")
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

	fn variable(&mut self) {
		self.named_variable(self.previous)
	}

	fn named_variable(&mut self, identifier: Token) {
		
		// I feel like these should be mutable but whatever
		let variable_index;
		let set_op;
		let get_op;

		let assignment = self.can_assign && self.current.ttype == TokenType::EQUAL;
		
		if let Some(index) = self.resolve_local(identifier, assignment) {
			set_op = SETLOCAL;
			get_op = GETLOCAL;
			variable_index = index;
		} else if let Some(index) = self.get_global(identifier, assignment) {
			set_op = SETGLOBAL;
			get_op = GETGLOBAL;
			variable_index = index;
		} else {
			self.error_at(identifier, "Cannot find variable");
			set_op = SETLOCAL;
			get_op = GETLOCAL;
			variable_index = 0;
		}

		if self.can_assign && self.current.ttype == TokenType::EQUAL {
			self.advance();
			self.expression();
			self.push_bytes(&[set_op, variable_index as u8]);
		} else {
			self.push_bytes(&[get_op, variable_index as u8]);
		}
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

	fn synchronize(&mut self) {
		self.panic = false;
		while self.current.ttype != TokenType::EOF {
			if self.previous.ttype == TokenType::SEMICOLON {return}
			match self.current.ttype {
				TokenType::CLASS 	|
				TokenType::FUN 		|
				TokenType::VAR 		|
				TokenType::FOR 		|
				TokenType::IF 		|
				TokenType::WHILE 	|
				TokenType::PRINT 	|
				TokenType::RETURN => return,
				_ => {}
			}
			self.advance();
		}
	}
}
