use token::TokenKind;

pub mod parser;
pub mod report;
pub mod scanner;
pub mod token;

pub struct AST {
    pub statements: Vec<ASTStatement>,
}

impl AST {
    pub fn new() -> Self {
        AST {
            statements: Vec::new(),
        }
    }
    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }
    // pub fn visit(visitor: &mut dyn ASTVisitor) {
    // }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
        }
    }
    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Binary(binary) => self.visit_binary_expression(binary),
            ASTExpressionKind::Group(group) => self.visit_group_expression(group),
            ASTExpressionKind::Number(n) => self.visit_number_expression(n),
        }
    }
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }
    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.do_visit_expression(expression);
    }
    fn visit_number_expression(&mut self, n: &ASTNumberExpression) {}
    fn visit_binary_expression(&mut self, binary: &ASTBinaryExpression) {}
    fn visit_group_expression(&mut self, group: &ASTGroupExpression) {}
    fn visit_literal_expression() {}
}
pub struct ASTStatement {
    kind: ASTStatementKind,
}
impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }
    pub fn expression(expression: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expression))
    }
}
pub enum ASTStatementKind {
    Expression(ASTExpression),
}

pub struct ASTExpression {
    kind: ASTExpressionKind,
}
impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(n: i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { number: n }))
    }

    pub fn binary(left: ASTExpression, operator: TokenKind, right: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Binary(ASTBinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    pub fn group(expr: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Group(ASTGroupExpression {
            expression: Box::new(expr),
        }))
    }
}

pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
    Binary(ASTBinaryExpression),
    Group(ASTGroupExpression),
}

pub struct ASTNumberExpression {
    number: i64,
}
pub struct ASTLiteralExpression {
    literal: String,
}
pub struct ASTBinaryExpression {
    left: Box<ASTExpression>,
    operator: TokenKind,
    right: Box<ASTExpression>,
}
pub struct ASTGroupExpression {
    expression: Box<ASTExpression>,
}
