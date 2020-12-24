use std::io;
use crate::scanner::{Scanner};

pub fn repl() {
	let mut buf = String::new();
	let stdin = io::stdin();
	loop {
		stdin.read_line(&mut buf).unwrap();
		let mut scanner = Scanner::new(buf.to_owned());
		let tokens = scanner.scan_all();
		println!("{:?}", tokens);
		buf = String::new();
	}
}
