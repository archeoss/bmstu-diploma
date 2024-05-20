use clap::Parser;
use std::fs;
use utoipa::OpenApi;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to save the OpenAPI schema
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let doc = bob_fusion_scheduler::ApiDoc::openapi().to_yaml().unwrap();
    let () = fs::write(args.filename, doc).expect("Couldn't write schema to file");
}
