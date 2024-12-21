use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Assignment(String, Box<Expr>),
}

impl Expr {
    pub fn generate_rust_code(&self) -> String {
        match self {
            Expr::Assignment(identifier, expr) => {
                format!("let {} = {};", identifier, expr.generate_rust_code())
            }
            Expr::Number(value) => value.to_string(),
            Expr::Identifier(name) => name.clone(),
            Expr::BinaryOp(left, op, right) => {
                format!("({} {} {})", left.generate_rust_code(), op, right.generate_rust_code())
            }
        }
    }
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Identifier(id) => write!(f, "{}", id),
            Expr::BinaryOp(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::Assignment(id, expr) => write!(f, "val {} = {}", id, expr),
        }
    }
}
