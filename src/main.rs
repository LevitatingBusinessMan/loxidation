mod chunk;
mod vm;
mod repl;
mod scanner;
use std::fs;
use crate::scanner::{Scanner};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() >= 1 {
        let source = fs::read_to_string(&args[0]).unwrap();
        let mut scanner = Scanner::new(source);
		let tokens = scanner.scan_all();
		println!("{:?}", tokens);

    } else {
        repl::repl();
    }
}
