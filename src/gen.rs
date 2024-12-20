use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Assignment(String, Box<Expr>),
}

impl Expr {
    // Converts the expression into the corresponding Rust code as a string
    pub fn generate_rust_code(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Identifier(id) => id.clone(),
            Expr::BinaryOp(lhs, op, rhs) => {
                format!("{} {} {}", lhs.generate_rust_code(), op, rhs.generate_rust_code())
            }
            Expr::Assignment(id, expr) => {
                format!("let {} = {};", id, expr.generate_rust_code())
            }
        }
    }
}

pub fn generate_rust_file(ast: &[Expr], file_path: &str) -> io::Result<()> {
    let mut code = String::new();
    
    // Add the Rust main function
    code.push_str("fn main() {\n");

    // Generate Rust code for each expression in the AST
    for expr in ast {
        code.push_str(&format!("    {}\n", expr.generate_rust_code()));
    }

    // Close the main function
    code.push_str("}\n");

    // Create or overwrite the output file
    let mut file = File::create(file_path)?;
    file.write_all(code.as_bytes())?;
    
    println!("Generated Rust file at: {}", file_path);

    Ok(())
}
