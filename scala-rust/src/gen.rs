use std::fs::File;
use std::io::{self, Write};
use crate::expr::Expr;

pub fn generate_rust_file(ast: &[Expr], file_path: &str) -> io::Result<()> {
    let mut code = String::new();
    
    code.push_str("fn main() {\n");

    for expr in ast {
        code.push_str(&format!("    {}\n", expr.generate_rust_code()));
    }
    code.push_str("}\n");
    
    let mut file = File::create(file_path)?;
    file.write_all(code.as_bytes())?;
    
    println!("Generated Rust file at: {}", file_path);

    Ok(())
}
