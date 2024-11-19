use token::{Token, TokenKind};

pub mod parser;
pub mod report;
pub mod scanner;
pub mod token;

pub struct AST {
    pub(crate) statements: Vec<Token>,
}
impl AST {
    pub fn new(statements: Vec<Token>) -> Self {
        AST { statements }
    }
}
pub struct ASTBinaryExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
    operator: TokenKind,
}

pub enum ASTExpressionKind {
    Binary(ASTBinaryExpression),
}
// * statement > expression
pub struct ASTExpression {
    kind: ASTExpressionKind,
}
impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn binary(
        left: ASTExpression,
        right: ASTExpression,
        operator: TokenKind,
    ) -> ASTBinaryExpression {
        ASTBinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}
