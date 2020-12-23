//https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations

use crate::chunk::Chunk;
use crate::chunk::value::{Value,ValueMethods};

pub type OpCode = u8;

//At first this was an enum with discriminant values
//But this would make it hard to add or remove an OP without shifting the values
pub const RETURN: u8 = 0x1;
pub const CONSTANT: u8 = 0x2;

//Possibly change the offset here to be a reference
pub fn disassemble(chunk: &Chunk, offset: usize) -> (String, usize) {
	let OP_offset = offset;
	let mut offset = offset;
	let op = chunk.code[offset];

	let line_number = chunk.lines[offset];
	let line_n_str = if offset > 0 && chunk.lines[offset-1] == line_number {
		"   |".to_owned()
	} else {
		format!("{:>4}", line_number)
	};	

	let name = match op {
		RETURN => "RETURN".to_owned(),
		CONSTANT => {
			offset+=1;
			let index = chunk.code[offset];
			let value = chunk.constants[index as usize];
			format!("{} {:04} ({})", "CONSTANT", index, value.print())
		},
		_ => "unknown".to_owned()
	};

	let line = format!("{} {:04} {}\n",line_n_str, OP_offset, name);
	return (line, offset+1)
}


/*

This is the old behavior using an enum instead of a module of constants

//You can use "as u8" for these opcodes
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
	RETURN = 0x1,
	CONSTANT
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "{:?}", self)
        // or, alternatively:
        std::fmt::Debug::fmt(self, f)
    }
}

impl OpCode {
	pub fn disassemble(&self, offset: u32) -> (String, u32) {

		/* let name = match self {
			OpCode::RETURN => "RETURN",
			_ => "unknown"
		}.to_owned() */
		let name = self.to_string();

		let line = format!("{:04} {}\n", offset, name);
		(line, offset + 1)
	}
}
*/
