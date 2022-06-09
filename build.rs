use tonic_build::{compile_protos, configure, Attributes, Builder, Method};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_protos("proto/root.proto")?;
    Ok(())
}
