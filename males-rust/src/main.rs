//! Pembangkit file korup dengan ukuran dan format sesuai pilihan pengguna.

mod cli;
mod generator;
mod magic;
mod rng;

use std::io::{self, Write};
use std::process::ExitCode;

fn run() -> io::Result<()> {
    let input = cli::collect()?;
    let format = magic::Format::from_extension(&input.extension);
    let filename = input.filename();

    print!("Mengisi data acak...");
    io::stdout().flush()?;

    let written = generator::generate_corrupt_file(&filename, input.size, format)?;
    println!();

    if format.is_some() {
        println!(
            "File korup '{}' ({written} byte) dengan magic bytes berhasil dibuat.",
            filename
        );
    } else {
        println!(
            "File korup '{}' ({written} byte) berhasil dibuat.",
            filename
        );
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
