use crate::lexer::{Token, Lexer};
use crate::expr::Expr;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn parse_number(&mut self) -> Expr {
        if let Token::Number(value) = &self.current_token {
            let value = *value;
            self.consume_token();
            Expr::Number(value)
        } else {
            panic!("Expected a number")
        }
    }

    fn parse_identifier(&mut self) -> Expr {
        if let Token::Identifier(ref name) = &self.current_token {
            let name = name.clone();
            self.consume_token();
            Expr::Identifier(name)
        } else {
            panic!("Expected an identifier")
        }
    }

    fn parse_assignment(&mut self) -> Expr {
        if let Token::Keyword(ref keyword) = &self.current_token {
            if keyword == "val" {
                self.consume_token(); // Consume "val"
                let identifier = self.parse_identifier(); // Parse identifier (x)
                if let Token::Punctuation(ref op) = &self.current_token {
                    if *op == '=' {
                        self.consume_token(); // Consume "="
                        let expr = self.parse_expression(); // Parse right-hand side expression (10)
                        return Expr::Assignment(identifier.to_string(), Box::new(expr));
                    }
                }
            }
        }
        panic!("Expected assignment")
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_term();

        while let Token::Punctuation(ref op) = &self.current_token {
            let operator = op.clone(); 
            self.consume_token(); 
            let right = self.parse_term();
            left = Expr::BinaryOp(Box::new(left), operator.to_string(), Box::new(right)); 
        }

        left
    }

    fn parse_term(&mut self) -> Expr {
        match &self.current_token {
            Token::Number(_) => self.parse_number(),
            Token::Identifier(_) => self.parse_identifier(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn consume_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}

pub fn parse_input(input: &str) -> Vec<Expr> {
    let lexer = Lexer::new(input); 
    let mut parser = Parser::new(lexer);
    let mut exprs = Vec::new();

    // loop until EOF
    while parser.current_token != Token::EndOfFile {
        println!("Current token: {:?}", parser.current_token); 

        if let Token::Keyword(ref keyword) = &parser.current_token {
            if keyword == "val" {
                let expr = parser.parse_assignment();
                println!("Parsed assignment: {:?}", expr); 
                exprs.push(expr);
                continue;  
            }
        }

        let expr = parser.parse_expression();
        println!("Parsed expression: {:?}", expr);  
        exprs.push(expr);

        parser.consume_token();
    }

    exprs
}
