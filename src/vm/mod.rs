pub mod chunk;
pub mod op_codes;
pub mod value;

use std::rc::Rc;

use self::chunk::Chunk;
use self::op_codes::*;
use self::value::{Value, number};

pub const STACK_SIZE: usize = 1024;

struct VM {
	chunk: Chunk,
	ip: usize,
	stack: Vec<Value>,
	globals: Vec<Value>,
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
		globals: vec![],
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
				return self.runtime_error("Binary operands must both be numbers or both be strings");
			}
			let b = number::from(pop!());
			let a = number::from(pop!());
			push!(Value::from(a $op b));
		}};}
		macro_rules! read_byte {() => {{
			self.ip += 1;
			self.chunk.code[self.ip-1]
		}};}
		macro_rules! read_word {() => {{
			self.ip += 2;
			((self.chunk.code[self.ip-2] as u16) << 8 | self.chunk.code[self.ip-1] as u16)
		}};}
		macro_rules! read_constant {() => {&self.chunk.constants[read_byte!() as usize]};}
		//#endregion

		return loop {
			
			#[cfg(debug_assertions)]
			self.print_stack();
			#[cfg(debug_assertions)]
			let (dis_str, _) = disassemble(&self.chunk, self.ip);
			#[cfg(debug_assertions)]
			eprint!("{}", dis_str);
			
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
						let b = Rc::<String>::from(pop!());
						let a = Rc::<String>::from(pop!());
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
					let index = read_byte!() as usize;
					if self.globals.len() <= index {
						self.globals.push(pop!());
					} else {
						self.globals[index] = pop!();
					}
				},
				GETGLOBAL => {
					let index = read_byte!() as usize;
					push!(self.globals[index].clone());
				},
				SETGLOBAL => {
					let index = read_byte!() as usize;
					let value = peek!(0).clone();
					if self.globals.len() <= index {
						self.globals.push(value);
					} else {
						self.globals[index] = value;
					}
				},
				GETLOCAL => {
					let index = read_byte!();
					// I could handle errors here but the compiler should make them impossible
					push!(self.stack[index as usize].clone());
				},
				SETLOCAL => {
					let index = read_byte!();
					// Don't pop, as an assignment is also an expression
					self.stack[index as usize] = peek!(0).clone();
				},
				JUMPIFFALSE => {
					let offset = read_word!() as i16;
					if !peek!(0).is_truthy() {
						self.ip = (self.ip as i64 +  offset as i64) as usize;
					}
				},
				JUMP => {
					let offset = read_word!() as i16;
					self.ip = (self.ip as i64 +  offset as i64) as usize;
				},
				LEAVE => {
					let n = read_byte!();
					self.stack.truncate(self.stack.len() - n as usize);
				},
				_ => return self.runtime_error(format!("Unknown opcode: {instruction:#x}"))
			}
		}
	}

	fn runtime_error(&self, msg: impl AsRef<str>) -> Result {
		let msg = msg.as_ref();
		let stack_length = self.chunk.code.len();
		if stack_length < 1 {
			eprintln!("Error: {}",msg);
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
			eprintln!("Error at line {}: {}",line,msg);
		}
		return Result::RUNTIME_ERROR(msg.to_owned());
	}

	pub fn print_stack(&self) {
		let mut str = String::with_capacity(self.stack.len() * 4);
		for value in &self.stack {
			str.push_str(&value.to_string());
			str.push_str(",");
		}
		eprintln!("[{}]",str);
	}

}
