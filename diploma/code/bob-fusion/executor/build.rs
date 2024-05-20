use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut proto = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto.pop();
    proto.pop();
    proto.push("api");
    let mut bob_proto = proto.clone();
    let mut executor_proto = proto.clone();
    bob_proto.push("bob.proto");
    executor_proto.push("executor.proto");
    tonic_build::configure()
        .build_server(true)
        .compile(&[executor_proto, bob_proto], &[proto])?;
    Ok(())
}
