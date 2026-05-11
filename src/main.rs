mod core;
mod unknown;

use crate::core::main::{repl_loop, run as source_run};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Execute code string
    #[arg(short = 'C', long)]
    code: Option<String>,

    /// Source file
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(code) = args.code {
        source_run(code);
    } else if let Some(file) = args.file {
        match std::fs::read_to_string(&file) {
            Ok(source) => source_run(source),
            Err(err) => eprintln!("Failed to read '{}': {}", file, err),
        }
    } else {
        repl_loop();
    }
}
