// src/parser.rs
use crate::lexer::Lexer;
use crate::token::TokenType;
use crate::ast::{
    Program, Statement, LetStatement, ReturnStatement, ExpressionStatement,
    Expression, Identifier, IntegerLiteral, PrefixExpression, InfixExpression
};

// PRECEDENCE LEVELS (Lowest to Highest)
#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

fn get_precedence(t: &TokenType) -> Precedence {
    match t {
        TokenType::Eq | TokenType::NotEq => Precedence::Equals,
        TokenType::LT | TokenType::GT => Precedence::LessGreater,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Slash | TokenType::Asterisk => Precedence::Product,
        TokenType::LParen => Precedence::Call,
        TokenType::LParen => Precedence::Call, // Ensure this maps to Call, not Lowest
        _ => Precedence::Lowest,
    }
}

pub struct Parser {
    l: Lexer,
    cur_token: TokenType,
    peek_token: TokenType,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(mut l: Lexer) -> Self {
        let cur = l.next_token();
        let peek = l.next_token();
        Parser { l, cur_token: cur, peek_token: peek, errors: vec![] }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };
        while self.cur_token != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    // --- STATEMENT PARSING ---

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        
        if !self.expect_peek_ident() { return None; }
        
        let name_val = match &self.cur_token {
            TokenType::Ident(s) => s.clone(),
            _ => return None,
        };
        let name = Identifier { token: self.cur_token.clone(), value: name_val };

        if !self.expect_peek(TokenType::Assign) { return None; }
        
        self.next_token(); // Skip '='

        // NOW WE PARSE THE EXPRESSION
        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token == TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Let(LetStatement { token, name, value }))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        self.next_token();

        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token == TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Return(ReturnStatement { token, return_value }))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token == TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Expression(ExpressionStatement { token, expression }))
    }

    // --- PRATT PARSER CORE ---

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_exp = match &self.cur_token {
            TokenType::Ident(_) => self.parse_identifier(),
            TokenType::Int(_) => self.parse_integer_literal(),
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(), // <--- NEW HOOK
            TokenType::Function => self.parse_function_literal(),
            TokenType::True | TokenType::False => self.parse_boolean(),
            _ => {
                self.no_prefix_parse_fn_error(self.cur_token.clone());
                return None;
            }
        };

        // 2. Infix Parsing (The loop handles operator precedence)
        while self.peek_token != TokenType::Semicolon && precedence < get_precedence(&self.peek_token) {
            match self.peek_token {
                TokenType::Plus | TokenType::Minus | TokenType::Slash | TokenType::Asterisk | 
TokenType::Eq | TokenType::NotEq | TokenType::LT | TokenType::GT => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                },
                // In parse_expression loop:
                TokenType::LParen => {
                    self.next_token();
                    left_exp = self.parse_call_expression(left_exp.unwrap());
                },
                _ => return left_exp
            }
        }
        left_exp
    }

    // --- PREFIX HANDLERS ---

    fn parse_identifier(&mut self) -> Option<Expression> {
        match &self.cur_token {
            TokenType::Ident(value) => Some(Expression::Identifier(Identifier {
                token: self.cur_token.clone(),
                value: value.clone(),
            })),
            _ => None,
        }
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        match &self.cur_token {
            TokenType::Int(value) => Some(Expression::IntegerLiteral(IntegerLiteral {
                token: self.cur_token.clone(),
                value: *value,
            })),
            _ => None,
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();

        // 1. Expect '(' after 'if' (e.g., if (x < y))
        // Note: We can make parens optional later, but let's enforce them for now for safety
        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        // Check for 'else'
        let mut alternative = None;
        if self.peek_token == TokenType::Else {
            self.next_token(); // Move to 'else'
            
            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }
            
            alternative = Some(self.parse_block_statement());
        }

        Some(Expression::If(crate::ast::IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    fn parse_block_statement(&mut self) -> crate::ast::BlockStatement {
        let token = self.cur_token.clone();
        let mut statements = vec![];

        self.next_token(); // Skip the '{'

        while self.cur_token != TokenType::RBrace && self.cur_token != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        crate::ast::BlockStatement { token, statements }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let operator = token.to_string();
        
        self.next_token();
        
        let right = self.parse_expression(Precedence::Prefix)?;
        
        Some(Expression::Prefix(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        }))
    }
    
    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest);
        
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }
        exp
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::Function(crate::ast::FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    // --- INFIX HANDLERS ---

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.cur_token.clone();
        let operator = token.to_string();
        let precedence = get_precedence(&self.cur_token);
        
        self.next_token();
        let right = self.parse_expression(precedence)?;
        
        Some(Expression::Infix(InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    // --- HELPERS ---

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token == t {
            self.next_token();
            true
        } else {
            self.peek_error(&t);
            false
        }
    }
    
    fn expect_peek_ident(&mut self) -> bool {
        match self.peek_token {
            TokenType::Ident(_) => { self.next_token(); true },
            _ => { self.errors.push(format!("Expected Ident, got {:?}", self.peek_token)); false }
        }
    }

    fn peek_error(&mut self, t: &TokenType) {
        self.errors.push(format!("Expected {:?}, got {:?}", t, self.peek_token));
    }
    
    fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        self.errors.push(format!("No prefix parse function for {:?}", t));
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut identifiers = vec![];

        // Empty params: fn()
        if self.peek_token == TokenType::RParen {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        // First param
        if let TokenType::Ident(value) = &self.cur_token {
            identifiers.push(Identifier {
                token: self.cur_token.clone(),
                value: value.clone(),
            });
        }

        // Subsequent params
        while self.peek_token == TokenType::Comma {
            self.next_token(); // skip ident
            self.next_token(); // skip comma

            if let TokenType::Ident(value) = &self.cur_token {
                identifiers.push(Identifier {
                    token: self.cur_token.clone(),
                    value: value.clone(),
                });
            }
        }

        if !self.expect_peek(TokenType::RParen) {
            return vec![];
        }

        identifiers
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let token = self.cur_token.clone();
        let arguments = self.parse_call_arguments();
        
        Some(Expression::Call(crate::ast::CallExpression {
            token,
            function: Box::new(function),
            arguments,
        }))
    }

    fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut args = vec![];

        if self.peek_token == TokenType::RParen {
            self.next_token();
            return args;
        }

        self.next_token();
        if let Some(arg) = self.parse_expression(Precedence::Lowest) {
            args.push(arg);
        }

        while self.peek_token == TokenType::Comma {
            self.next_token();
            self.next_token();
            if let Some(arg) = self.parse_expression(Precedence::Lowest) {
                args.push(arg);
            }
        }

        if !self.expect_peek(TokenType::RParen) {
            return vec![];
        }

        args
    }
    // Add this method to the Parser struct
    fn parse_boolean(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let value = match token {
            TokenType::True => true,
            TokenType::False => false,
            _ => return None,
        };
        
        Some(Expression::Boolean(crate::ast::BooleanLiteral { token, value }))
    }
}