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
		let chunk = compile(buf).unwrap();
		println!("{}", chunk.disassemble("REPL"));
        vm::interpret(chunk);
		buf = String::new();
	}
}
