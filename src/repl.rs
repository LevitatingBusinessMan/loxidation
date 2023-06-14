use std::io;
use std::io::prelude::Write;
use crate::compiler::compile;
use crate::vm;

pub fn repl() {
	let mut buf = String::new();
	let stdin = io::stdin();
	loop {
		print!("lox> ");
		io::stdout().flush().unwrap();
		stdin.read_line(&mut buf).unwrap();
		if let Ok(chunk) = compile(buf.to_string()) {
			#[cfg(debug_assertions)]
			eprintln!("{}", chunk.disassemble("REPL"));
			vm::interpret(chunk);
		}
		buf = String::new();
	}
}
