//https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations

//You can use "as u8" for these opcodes
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
	RETURN = 0x1
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

//pub const RETURN: u8 = 0x1;

//pub type OpCode = u8;

/*
impl OpCode {
	fn code(&self) -> u8 {
		match *self {
			OpCode::RETURN => RETURN
		}
	}
}*/
