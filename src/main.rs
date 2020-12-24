mod chunk;
use chunk::Chunk;
use chunk::op_codes;

mod vm;
use vm::{VM,interpret};

fn main() {
    let mut new_chunk = Chunk::new();
    
    let constant_index  = new_chunk.push_constant(1.2);
    new_chunk.push_op(op_codes::CONSTANT, 1);
    new_chunk.push_op(constant_index as u8, 1);

    let constant_index  = new_chunk.push_constant(3.4);
    new_chunk.push_op(op_codes::CONSTANT, 1);
    new_chunk.push_op(constant_index as u8, 1);

    new_chunk.push_op(op_codes::ADD, 1);

    let constant_index  = new_chunk.push_constant(5.6);
    new_chunk.push_op(op_codes::CONSTANT, 1);
    new_chunk.push_op(constant_index as u8, 1);

    new_chunk.push_op(op_codes::DIVIDE, 1);

    new_chunk.push_op(op_codes::NEGATE, 1);

    new_chunk.push_op(op_codes::RETURN, 1);

    println!("{}", new_chunk.disassemble("new_chunk"));

    let result = interpret(new_chunk);
    println!("{:?}", result);
}
