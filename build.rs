use std::io::Result;
fn main() -> Result<()> {
    // prost_build::compile_protos(&["src/protos/transaction.buffer.proto", "src/protos/block.buffer.proto"], &["src/"])?;
    tonic_build::compile_protos("src/protos/transaction.proto")?;
    tonic_build::compile_protos("src/protos/block.proto")?;
    Ok(())
}