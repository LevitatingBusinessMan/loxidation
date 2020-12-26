use crate::chunk::Chunk;
use crate::chunk::op_codes::*;
use crate::chunk::value::{Value,ValueMethods};

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
		macro_rules! pop { () => {self.stack.pop().unwrap()};}
		macro_rules! push {($value:expr) => {self.stack.push($value)};}
		macro_rules! binary_op {($op:tt) => {{
			let b = pop!();
			let a = pop!();
			push!(a $op b);
		}};}
		macro_rules! read_byte {() => {{
			self.ip += 1;
			self.chunk.code[self.ip-1]
		}};}
		macro_rules! read_constant {() => {self.chunk.constants[read_byte!() as usize]};}
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
					println!("{}\n", pop!());
					break Result::OK;
				},
				CONSTANT => push!(read_constant!()),
				NEGATE => {
					let new = -pop!();push!(new);
					//push!(-pop!()) cause second mutable borrow
				},
				ADD => binary_op!(+),
				SUBTRACT => binary_op!(-),
				MULTIPLY => binary_op!(*),
				DIVIDE => binary_op!(/),
				_ => break Result::RUNTIME_ERROR
			}
		}
	}

	pub fn print_stack(&self) {
		let mut str = String::with_capacity(self.stack.len() * 4);
		for value in &self.stack {
			str.push_str(&value.print());
			str.push_str(",");
		}
		println!("[{}]",str);
	}

}
