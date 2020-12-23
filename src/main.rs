mod chunk;
use chunk::Chunk;
use chunk::op_codes;

mod vm;
use vm::{VM,interpret};

fn main() {
    let mut new_chunk = Chunk::new();
    
    let constant_index  = new_chunk.push_constant(1.2);
    new_chunk.push_op(op_codes::CONSTANT, 123);
    new_chunk.push_op(constant_index as u8, 123);
    new_chunk.push_op(op_codes::RETURN, 123);

    println!("{}", new_chunk.disassemble("new_chunk"));

    let vm = interpret(new_chunk);
}
