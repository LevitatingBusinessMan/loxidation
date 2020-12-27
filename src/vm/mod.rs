use crate::chunk::Chunk;
use crate::chunk::op_codes::*;
use crate::chunk::value::{Value, number};

pub const STACK_SIZE: usize = 1024;

struct VM {
	chunk: Chunk,
	ip: usize,
	stack: Vec<Value>
}

#[derive(Debug)]
pub enum Result {
	OK,
	COMPILE_ERROR,
	RUNTIME_ERROR
}

pub fn interpret(chunk: Chunk) -> Result {
	let mut vm = VM{
		chunk,
		ip: 0,
		stack: Vec::with_capacity(STACK_SIZE)
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
				self.runtime_error("Binary operands must be a numbers");
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
					println!("{}\n", pop!().to_string());
					break Result::OK;
				},
				CONSTANT => push!(read_constant!().clone()),
				NEGATE => {
					if matches!(peek!(0), Value::NUMBER(_)) {
						//Grab the number, negate and convert back
						let new = Value::from(-(number::from(pop!())));
						push!(new);
					} else {
						self.runtime_error("Operand must be a number");
					}
				},
				ADD => binary_op!(+),
				SUBTRACT => binary_op!(-),
				MULTIPLY => binary_op!(*),
				DIVIDE => binary_op!(/),
				_ => break Result::RUNTIME_ERROR
			}
		}
	}

	fn runtime_error(&self, msg: impl AsRef<str>) {
		let msg = msg.as_ref();
		let stack_length = self.chunk.code.len();
		if stack_length < 1 {
			return eprintln!("[ERROR no-line]: {}",msg);
		}
		let (_, line) = disassemble(&self.chunk, stack_length-1);
		eprintln!("[ERROR {}]: {}",line,msg);
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
