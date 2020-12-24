mod chunk;
mod vm;
mod repl;
mod scanner;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 1 {
        //readfile
    } else {
        repl::repl();
    }
}
