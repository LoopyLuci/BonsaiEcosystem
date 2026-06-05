//! Universal Driver Compiler - CLI binary

use bonsai_udc::{Cli, CliArgs, ConversionEngine};
use std::io;

fn main() {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Show help if no args
    if args.len() < 2 {
        println!("Universal Driver Compiler (UDC) - CLI");
        println!("Usage: {} <command> [options]", args[0]);
        println!("Run with 'help' for detailed instructions");
        return;
    }

    // Parse arguments
    let cli_args = match CliArgs::parse(&args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return;
        }
    };

    // Create CLI with default engine
    let engine = ConversionEngine::with_default_rules();
    let mut cli = Cli::new(engine);

    // Execute command
    match cli.execute(&cli_args) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}
