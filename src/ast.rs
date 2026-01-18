use crate::token::TokenType;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

// Added PartialEq
#[derive(Debug, Clone, PartialEq)] 
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
    fn string(&self) -> String {
        self.statements.iter().map(|s| s.string()).collect()
    }
}

// Added PartialEq
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token.to_string(),
            Statement::Return(s) => s.token.to_string(),
            Statement::Expression(s) => s.token.to_string(),
        }
    }
    fn string(&self) -> String {
        match self {
            Statement::Let(s) => format!("let {} = {};", s.name.value, s.value.string()),
            Statement::Return(s) => format!("return {};", s.return_value.string()),
            Statement::Expression(s) => s.expression.string(),
        }
    }
}

// --- EXPRESSIONS ---

// Added PartialEq
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Boolean(BooleanLiteral), 
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
    Function(FunctionLiteral),
    Call(CallExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(e) => e.token.to_string(),
            Expression::IntegerLiteral(e) => e.token.to_string(),
            Expression::Boolean(e) => e.token.to_string(),
            Expression::Prefix(e) => e.token.to_string(),
            Expression::Infix(e) => e.token.to_string(),
            Expression::If(e) => e.token.to_string(),
            Expression::Function(e) => e.token.to_string(),
            Expression::Call(e) => e.token.to_string(),
        }
    }
    fn string(&self) -> String {
        match self {
            Expression::Identifier(e) => e.value.clone(),
            Expression::IntegerLiteral(e) => e.value.to_string(),
            Expression::Boolean(e) => e.token.to_string(),
            Expression::Prefix(e) => format!("({}{})", e.operator, e.right.string()),
            Expression::Infix(e) => format!("({} {} {})", e.left.string(), e.operator, e.right.string()),
            Expression::If(e) => {
                let mut out = format!("if {} {{ {} }}", e.condition.string(), e.consequence.string());
                if let Some(alt) = &e.alternative {
                    out.push_str(&format!(" else {{ {} }}", alt.string()));
                }
                out
            },
            Expression::Function(e) => {
                let params: Vec<String> = e.parameters.iter().map(|p| p.value.clone()).collect();
                format!("fn({}) {}", params.join(", "), e.body.string())
            },
            Expression::Call(e) => {
                let args: Vec<String> = e.arguments.iter().map(|a| a.string()).collect();
                format!("{}({})", e.function.string(), args.join(", "))
            },
        }
    }
}

// --- STRUCTS ---

// Added PartialEq to ALL structs below

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub token: TokenType,
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub token: TokenType,
    pub return_value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub token: TokenType,
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub token: TokenType,
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String { self.token.to_string() }
    fn string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            out.push_str(&s.string());
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression {
    pub token: TokenType,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub token: TokenType,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub token: TokenType,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub token: TokenType,
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixExpression {
    pub token: TokenType, 
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfixExpression {
    pub token: TokenType, 
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionLiteral {
    pub token: TokenType,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub token: TokenType,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}