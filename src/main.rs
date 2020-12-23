mod chunk;
use chunk::Chunk;
use chunk::op_codes;

fn main() {
    let mut new_chunk = Chunk::new();
    new_chunk.code.push(op_codes::RETURN);
    
    let constant_index  = new_chunk.push_constant(1.2);
    new_chunk.push_op(op_codes::CONSTANT, 123);
    new_chunk.push_op(constant_index as u8, 123);

    println!("{}", new_chunk.disassemble("new_chunk"));
}
