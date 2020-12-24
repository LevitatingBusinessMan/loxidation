mod chunk;
use chunk::Chunk;
use chunk::op_codes;

mod vm;
use vm::{VM,interpret};

fn main() {
    let mut new_chunk = Chunk::new();
    
    let constant_index  = new_chunk.push_constant(1.2);
    let constant_index2  = new_chunk.push_constant(2.4);
    new_chunk.push_op(op_codes::CONSTANT, 123);
    new_chunk.push_op(constant_index as u8, 123);
    new_chunk.push_op(op_codes::CONSTANT, 123);
    new_chunk.push_op(constant_index2 as u8, 123);
    new_chunk.push_op(op_codes::ADD, 123);
    new_chunk.push_op(op_codes::RETURN, 124);

    println!("{}", new_chunk.disassemble("new_chunk"));

    let result = interpret(new_chunk);
    println!("{:?}", result);
}
