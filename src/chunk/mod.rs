pub mod op_codes;
pub mod value;

use value::Value;
use op_codes::OpCode;

//Note, the current implementation in rust makes a vector double capacity when full
pub struct Chunk {
	pub code: Vec<OpCode>,
	pub constants: Vec<Value>,
	pub lines: Vec<u32>
}

impl Chunk {
	pub fn new() -> Chunk {
		Chunk{
			code: Vec::<OpCode>::new(),
			constants: Vec::<Value>::new(),
			lines: Vec::<u32>::new()
		}
	}
	pub fn disassemble(&self, name: &str) -> String {
		//header
		let mut str = format!("== {} ==\n", name);

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
	pub fn push_constant(&mut self, constant: f64) -> usize {
		self.constants.push(constant);
		self.constants.len() -1
	}

	pub fn push_op(&mut self, op: OpCode, line: u32) -> usize {
		self.code.push(op);
		self.lines.push(line);
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
