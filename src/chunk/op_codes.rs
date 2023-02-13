//https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations

use crate::chunk::Chunk;

///Code for an operator or the value of an operand
pub type OpCode = u8;

//At first this was an enum with discriminant values
//The benefit was that the name could be derived from the variant
//but it was hard to convert the type to a u8.
//#region
pub const RETURN: OpCode = 0x1;
pub const CONSTANT: OpCode = 0x2;
pub const NEGATE: OpCode = 0x3;
pub const ADD: OpCode = 0x4;
pub const SUBTRACT: OpCode = 0x5;
pub const MULTIPLY: OpCode = 0x6;
pub const DIVIDE: OpCode = 0x7;
pub const NIL: OpCode = 0x8;
pub const TRUE: OpCode = 0x9;
pub const FALSE: OpCode = 0xa;
pub const NOT: OpCode = 0xb;
pub const EQUAL: OpCode = 0xc;
pub const GREATER: OpCode = 0xd;
pub const LESS: OpCode = 0xe;
pub const PRINT: OpCode = 0xf;
pub const POP: OpCode = 0x10;
pub const DEFGLOBAL: OpCode = 0x11;
pub const GETGLOBAL: OpCode = 0x12;
pub const SETGLOBAL: OpCode = 0x13;
//#endregion

//Possibly change the offset here to be a reference
pub fn disassemble(chunk: &Chunk, offset: usize) -> (String, usize) {
	let op_offset = offset;
	let mut offset = offset;
	let op = chunk.code[offset];

	let mut i = 0;
	let mut previous = &chunk.lines[0];
	let line_n_str: String = loop {
		let line = &chunk.lines[i];
		if line.length >= offset {

			//First op has no previous line struct
			//to see if it is in fact the first of its line
			if op_offset == 0 {
				break format!("{:>4}", line.number);
			}
			if i > 0 {
				//let previous = &chunk.lines[i-1];
				if previous.length+1 == offset {
					break format!("{:>4}", line.number);
				}
			}
			break "   |".to_owned();
		}
		previous = line;
		i += 1;
	};

	/*let line_n_str = if line.length >= offset {
		"   |".to_owned()
	} else {
		format!("{:>4}", line.number)
	};*/	

	let name = match op {
		CONSTANT => {
			offset+=1;
			let index = chunk.code[offset];
			let value = &chunk.constants[index as usize];
			format!("{} {:04} ({})", "CONSTANT", index, value.to_string())
		},
		DEFGLOBAL => {
			offset+=1;
			let index = chunk.code[offset];
			let value = &chunk.constants[index as usize];
			format!("{} {:04} ({})", "DEFGLOBAL", index, value.to_string())
		},
		GETGLOBAL => {
			offset+=1;
			let index = chunk.code[offset];
			let value = &chunk.constants[index as usize];
			format!("{} {:04} ({})", "GETGLOBAL", index, value.to_string())
		},
		SETGLOBAL => {
			offset+=1;
			let index = chunk.code[offset];
			let value = &chunk.constants[index as usize];
			format!("{} {:04} ({})", "SETGLOBAL", index, value.to_string())
		}
		_ => {
			match op {
				RETURN => "RETURN",
				NEGATE => "NEGATE",
				ADD => "ADD",
				SUBTRACT => "SUBTRACT",
				MULTIPLY => "MULTIPLY",
				DIVIDE => "DIVIDE",
				NIL => "NIL",
				TRUE => "TRUE",
				NOT => "NOT",
				EQUAL => "EQUAL",
				GREATER => "GREATER",
				LESS => "LESS",
				PRINT => "PRINT",
				POP => "POP",
				_ => "unknown"
			}.to_owned()
		}
	};

	let line = format!("{} {:04} {}\n",line_n_str, op_offset, name);
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
//Removed because removing an op would shift all ints
//Along with some other inconveniences (like difficulty converting to u8)

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
