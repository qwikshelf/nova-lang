use crate::token::{TokenType, lookup_ident};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        let tok = match self.ch {
            // NEW: Handle == and =
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenType::Eq
                } else {
                    TokenType::Assign
                }
            },
            // NEW: Handle != and !
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenType::NotEq
                } else {
                    TokenType::Bang
                }
            },
            // NEW: Handle < and >
            '<' => TokenType::LT,
            '>' => TokenType::GT,

            '+' => TokenType::Plus,
            '-' => {
                if self.peek_char() == '>' {
                    self.read_char();
                    TokenType::Arrow
                } else {
                    TokenType::Minus
                }
            },
            '/' => TokenType::Slash,
            '*' => TokenType::Asterisk,
            ',' => TokenType::Comma,
            ';' => TokenType::Semicolon,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '\0' => TokenType::EOF,
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return lookup_ident(&literal);
                } else if is_digit(self.ch) {
                    let literal = self.read_number();
                    return TokenType::Int(literal.parse().unwrap());
                } else {
                    TokenType::Illegal
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_numeric()
}