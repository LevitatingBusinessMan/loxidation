#![feature(try_trait_v2)]
mod vm;
mod repl;
mod scanner;
mod compiler;
use std::fs;
use compiler::compile;
use scanner::Scanner;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() >= 1 {
        let source = fs::read_to_string(&args[0]).unwrap();
        if let Ok(chunk) = compile(source) {
            #[cfg(debug_assertions)]
            eprintln!("{}", chunk.disassemble(&args[0]));
            vm::interpret(chunk);
        }
    } else {
        repl::repl();
    }
}
