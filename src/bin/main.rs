use std::env;
use std::fs;
use scala_rust::gen::{generate_rust_file, Expr};
use scala_rust::lexer::{Lexer, Token};
use scala_rust::parser::parse_input;

fn main() -> std::io::Result<()> {
    // File path
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "tests/source.scala"
    };

    // Read the source code from the file
    let source_code = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file: {}", filename);
        std::process::exit(1);
    });

    // Create a lexer and tokenize the source code
    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();
    println!("Tokens: {:?}", tokens);

    // Parse the input to get the AST
    let parsed = parse_input(&source_code);
    println!("Parsed AST: {:?}", parsed);

    // Generate the Rust file based on the parsed AST
    let file_path = "generated_code.rs"; // Output Rust file path
    generate_rust_file(&parsed[..], file_path)?;


    Ok(())
}

