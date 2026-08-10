//! Runs a local box through the typed Rust consumer.
//!
//! SETUP (once, from the project root):
//!   cargo add --manifest-path consumer-templates/rust/Cargo.toml scrollcase-consumer
//!
//! RUN:
//!   cargo run --manifest-path consumer-templates/rust/Cargo.toml
//!
//! Replace <target> and <hash> below with the values printed by scrollcase build.

use std::error::Error;
use std::path::Path;

use scrollcase_consumer::run::{run_box, RunBoxOptions, RunOptions};
use scrollcase_consumer::trust::TrustAnchors;

const RELEASE_TO_RUN: &str =
    ".scrollcase/dist/boxes/example-box/1.0.0/<target>/<hash>.release.json";

fn main() -> Result<(), Box<dyn Error>> {
    let temporary_root = std::env::temp_dir();
    let result = run_box(
        Path::new(RELEASE_TO_RUN),
        &RunBoxOptions {
            trust: TrustAnchors::KeyFile(Path::new(".scrollcase/keys/signing-public.json")),
            archive: None,
            temporary_root: &temporary_root,
            run: RunOptions::default(),
        },
    )?;

    if let Some(signal) = result.signal {
        eprintln!("Box exited after {signal}.");
    }
    std::process::exit(result.exit_code.unwrap_or(1));
}
