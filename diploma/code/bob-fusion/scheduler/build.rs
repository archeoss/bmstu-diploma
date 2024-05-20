use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut protofile = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    protofile.pop();
    protofile.pop();
    protofile.push("api");
    protofile.push("scheduler.proto");
    tonic_build::compile_protos(protofile)?;
    Ok(())
}
