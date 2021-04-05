mod chunk;
mod vm;
mod repl;
mod scanner;
mod compiler;
use std::fs;
use compiler::compile;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() >= 1 {
        let source = fs::read_to_string(&args[0]).unwrap();
        let chunk = compile(source).unwrap();
        #[cfg(debug_assertions)]
        println!("{}", chunk.disassemble(&args[0]));
        vm::interpret(chunk);

    } else {
        repl::repl();
    }
}
