use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Operator(String),
    Number(i64),
    StringLiteral(String),
    Punctuation(char),
    Whitespace,
    EndOfFile,
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, current_pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.current_pos..].chars().next()
    }

    fn consume(&mut self) -> Option<char> {
        let char = self.peek();
        if let Some(c) = char {
            self.current_pos += c.len_utf8();
        }
        char
    }

    // Lex the next token.
    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\n' | '\r' | '\t' => {
                    self.consume();
                    continue;
                }

                'a'..='z' | 'A'..='Z' => {
                    let start_pos = self.current_pos;
                    while let Some(c) = self.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            self.consume();
                        } else {
                            break;
                        }
                    }
                    let identifier = &self.input[start_pos..self.current_pos];
                    if ["if", "else", "while", "val"].contains(&identifier) {
                        return Token::Keyword(identifier.to_string());
                    } else {
                        return Token::Identifier(identifier.to_string());
                    }
                }

                // Handle numbers.
                '0'..='9' => {
                    let start_pos = self.current_pos;
                    while let Some(c) = self.peek() {
                        if c.is_digit(10) {
                            self.consume();
                        } else {
                            break;
                        }
                    }
                    let number = &self.input[start_pos..self.current_pos];
                    return Token::Number(i64::from_str(number).unwrap());
                }

                '"' => {
                    self.consume();
                    let start_pos = self.current_pos;
                    while let Some(c) = self.peek() {
                        if c == '"' {
                            break;
                        }
                        self.consume();
                    }
                    let string_literal = &self.input[start_pos..self.current_pos];
                    self.consume(); // consume closing "
                    return Token::StringLiteral(string_literal.to_string());
                }

                '+' | '-' | '*' | '/' | '=' | ';' | '(' | ')' | '{' | '}' => {
                    return Token::Punctuation(self.consume().unwrap());
                }

                _ => break,
            }
        }

        Token::EndOfFile
    }

    // Tokenize the entire input.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::EndOfFile {
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}
