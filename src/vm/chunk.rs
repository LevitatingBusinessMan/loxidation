use crate::vm::value::Value;
use crate::vm::op_codes;
use crate::vm::op_codes::OpCode;

//Note, the current implementation in rust makes a vector double capacity when full
/// A Chunk is compiled by the compiler and used by the VM
/// It holds everything necessary to run the code and produce runtime error messages
pub struct Chunk {
	pub code: Vec<OpCode>,
	pub constants: Vec<Value>,
	pub lines: Vec<Line>
}

/// The linenumber and the index of the last op of this line
pub struct Line {
	pub number: u32,
	pub length: usize
}

impl Chunk {
	pub fn new() -> Chunk {
		Chunk{
			code: Vec::<OpCode>::new(),
			constants: Vec::<Value>::new(),
			lines: Vec::<Line>::new()
		}
	}
	pub fn disassemble(&self, name: &str) -> String {
		//header
		let mut str = format!("== {} ==\n", name);

		// Uncomment this to show constant table
		// let mut i = 0;
		// while i < self.constants.len() {
		// 	let constant = &self.constants[i];
		// 	str.push_str(&format!("constant {}: {}\n", i, constant.to_string()));
		// 	i += 1;
		// }

		let mut offset = 0;
		while offset < self.code.len() {
			let (line, new_offset) = op_codes::disassemble(self, offset);
			offset = new_offset as usize;
			str.push_str(line.as_str());
		}

		str
	}

	//Pushes the constant and gives the index
	//Really unnecesarry but it saves a line
	pub fn push_constant(&mut self, constant: Value) -> usize {
		self.constants.push(constant);
		self.constants.len() -1
	}

	pub fn push_op(&mut self, op: OpCode, line: u32) -> usize {
		self.code.push(op);

		let last_line = self.lines.last_mut();
		
		match last_line {
			Some(last_line) => {
				if last_line.number == line {
					last_line.length += 1;
				} else {
					self.lines.push(Line{number: line, length: self.code.len() -1})
				}
			},
			//First op
			None => self.lines.push(Line{number: line, length: 0})
		}

		return self.code.len() -1
	}

}

//pub type Chunk = Vec<OpCode>;

/* //I can't impl a method directly because this is an alias type
pub trait DisassembleChunk {
	fn disassemble(&self, name: &str) -> String;
}

impl DisassembleChunk for Chunk {
	fn disassemble(&self, name: &str) -> String {
		
		//header
		let mut str = format!("== {} ==\n", name);

		let mut offset = 0;
		while offset < self.len() {
			let (line, new_offset) = self[offset].disassemble(offset as u32);
			offset = new_offset as usize;
			str.push_str(line.as_str());
		}

		str
	}
} */

/*
If I want methods I can also use a newtype but this will mess with the vector methods

pub struct Chunk(Vec<OpCode>);

impl Chunk {
	pub fn new() -> Chunk {
		return Chunk(Vec::<OpCode>::new());
	}
}
*/
