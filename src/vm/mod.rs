use crate::chunk::Chunk;
use crate::chunk::op_codes::*;
use crate::chunk::value::{Value, number};
use std::collections::HashMap;

pub const STACK_SIZE: usize = 1024;

struct VM {
	chunk: Chunk,
	ip: usize,
	stack: Vec<Value>,
	globals: HashMap<String, Value>
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Result {
	OK,
	RUNTIME_ERROR(String)
}

pub fn interpret(chunk: Chunk) -> Result {
	let mut vm = VM{
		chunk,
		ip: 0,
		stack: Vec::with_capacity(STACK_SIZE),
		globals: HashMap::new()
	};
	vm.run()
}

impl VM {

	fn run(&mut self) -> Result {

		//#region
		//Defining these macro's outside this function would
		//make them need "self" as an argument
		macro_rules! peek { ($distance:expr) => {self.stack[self.stack.len()-1 - $distance]}}
		macro_rules! pop { () => {self.stack.pop().unwrap()};}
		macro_rules! push {($value:expr) => {self.stack.push($value)};}
		macro_rules! binary_op {($op:tt) => {{
			if !matches!(peek!(0), Value::NUMBER(_)) || !matches!(peek!(1), Value::NUMBER(_)) {
				return self.runtime_error("Binary operands must be both numbers or both strings");
			}
			let b = number::from(pop!());
			let a = number::from(pop!());
			push!(Value::from(a $op b));
		}};}
		macro_rules! read_byte {() => {{
			self.ip += 1;
			self.chunk.code[self.ip-1]
		}};}
		macro_rules! read_constant {() => {&self.chunk.constants[read_byte!() as usize]};}
		//#endregion

		return loop {
			
			#[cfg(debug_assertions)]
			self.print_stack();
			#[cfg(debug_assertions)]
			let (dis_str, _) = disassemble(&self.chunk, self.ip);
			#[cfg(debug_assertions)]
			print!("{}", dis_str);
			
			let instruction = read_byte!();
			match instruction {
				RETURN => {
					//println!("{}\n", pop!());
					break Result::OK;
				},
				CONSTANT => push!(read_constant!().clone()),
				NEGATE => {
					if matches!(peek!(0), Value::NUMBER(_)) {
						//Grab the number, negate and convert back
						let new = Value::from(-(number::from(pop!())));
						push!(new);
					} else {
						return self.runtime_error("Operand must be a number");
					}
				},
				SUBTRACT => binary_op!(-),
				MULTIPLY => binary_op!(*),
				DIVIDE => binary_op!(/),
				GREATER => binary_op!(>),
				LESS => binary_op!(<),
				ADD => {
					if matches!(peek!(0), Value::STRING(_)) && matches!(peek!(1), Value::STRING(_)) {
						let b = String::from(pop!());
						let a = String::from(pop!());
						push!(Value::from(format!("{}{}",a,b)));
					} else {
						binary_op!(+);
					}
				},
				NIL => push!(Value::NIL),
				TRUE => push!(Value::BOOL(true)),
				FALSE => push!(Value::BOOL(false)),
				NOT => {
					let new = !pop!().is_truthy();
					push!(Value::from(new));
				},
				EQUAL => {
					let b = pop!();
					let a = pop!();
					push!(Value::BOOL(a.equal(b)));
				},
				PRINT => {
					println!("{}",pop!());
				},
				POP => {
					pop!();
				},
				DEFGLOBAL => {
					let identifier_constant = read_constant!().clone();
					if let Value::STRING(identifier) = identifier_constant {
						self.globals.insert(identifier, pop!());
					} else {
						return self.runtime_error(format!("Identifier for global variable isn't of type string: {:?}", identifier_constant))
					}
				},
				GETGLOBAL => {
					let identifier_constant = read_constant!().clone();
					if let Value::STRING(identifier) = identifier_constant {
						if let Some(value) = self.globals.get(&identifier) {
							push!(value.clone()) // ye this is where shit gets tough
						} else {
							return self.runtime_error(format!("Global variable {:?} doesn't exist", identifier))
						}
					} else {
						return self.runtime_error(format!("Identifier for global variable isn't of type string: {:?}", identifier_constant))
					}
				},
				SETGLOBAL => {
					let identifier_constant = read_constant!().clone();
					if let Value::STRING(identifier) = identifier_constant {
						if self.globals.contains_key(&identifier, ) {
							self.globals.insert(identifier, peek!(0).clone());
						} else {
							return self.runtime_error(format!("Undefined variable: {:?}", identifier))
						}
					} else {
						return self.runtime_error(format!("Identifier for global variable isn't of type string: {:?}", identifier_constant))
					}
				},
				_ => return self.runtime_error(format!("Unknown opcode: 0x{}", std::char::from_digit(instruction as u32, 16).unwrap()))
			}
		}
	}

	fn runtime_error(&self, msg: impl AsRef<str>) -> Result {
		let msg = msg.as_ref();
		let stack_length = self.chunk.code.len();
		if stack_length < 1 {
			eprintln!("[ERROR no-line]: {}",msg);
		} else {
			let  mut i = 0;
			let offset = stack_length-1;
			let line = loop {
				let line = &self.chunk.lines[i];
				if line.length >= offset {
					break line.number;
				}
				i += 1;
			};
			eprintln!("[ERROR {}]: {}",line,msg);
		}
		return Result::RUNTIME_ERROR(msg.to_owned());
	}

	pub fn print_stack(&self) {
		let mut str = String::with_capacity(self.stack.len() * 4);
		for value in &self.stack {
			str.push_str(&value.to_string());
			str.push_str(",");
		}
		println!("[{}]",str);
	}

}
