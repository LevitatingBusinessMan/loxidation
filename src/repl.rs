use std::io;
use crate::compiler::compile;

pub fn repl() {
	let mut buf = String::new();
	let stdin = io::stdin();
	loop {
		stdin.read_line(&mut buf).unwrap();
		let chunk = compile(buf).unwrap();
		chunk.disassemble("REPL");
		buf = String::new();
	}
}
