pub mod op_codes;

use op_codes::OpCode;


//Note, the current implementation in rust makes a vector double capacity when full
pub type Chunk = Vec<OpCode>;

//I can't impl a method directly because this is an alias type
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
}

/*
If I want methods I can also use a newtype but this will mess with the vector methods

pub struct Chunk(Vec<OpCode>);

impl Chunk {
	pub fn new() -> Chunk {
		return Chunk(Vec::<OpCode>::new());
	}
}
*/
