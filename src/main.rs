mod chunk;
use chunk::Chunk;
use chunk::op_codes::OpCode;
use chunk::DisassembleChunk;

fn main() {
    let mut new_chunk = Chunk::new();
    new_chunk.push(OpCode::RETURN);
    println!("{}", new_chunk.disassemble("new_chunk"));
}
