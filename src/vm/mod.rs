use crate::chunk::Chunk;
use crate::chunk::op_codes::*;
use crate::chunk::value::{Value,ValueMethods};

pub struct VM {
	chunk: Chunk,
	ip: usize
}

enum Result {
	OK,
	COMPILE_ERROR,
	RUNTIME_ERROR
}

pub fn interpret(chunk: Chunk) {
	let mut vm = VM{
		chunk,
		ip: 0
	};
	vm.run();
}

impl VM {

	fn run(&mut self) -> Result {
		return loop {
			let instruction = self.read_byte();
			match instruction {
				RETURN => {
					break Result::OK;
				},
				CONSTANT => {
					let constant = self.read_constant();
					println!("{}\n",constant.print());
				},
				_ => break Result::RUNTIME_ERROR
			}
		}
	}

	fn read_byte(&mut self) -> u8 {
		let byte = self.chunk.code[self.ip];
		self.ip += 1;
		return byte;
	}

	fn read_constant(&mut self) -> Value {
		let byte = self.read_byte() as usize;
		self.chunk.constants[byte]
	}

}
