use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // * single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bad,
    // * one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // *
    Identifier,
    String,
    Number(i64),
    // *
    And,
    Class,
    If,
    Else,
    True,
    False,
    Function,
    Return,
    For,
    Console,
    Var,
    Whitespace,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}
impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        TextSpan {
            start,
            end,
            literal,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) line: u32,
    pub(crate) span: TextSpan,
}
impl Token {
    pub fn new(kind: TokenKind, line: u32, span: TextSpan) -> Self {
        Token { kind, line, span }
    }
}

pub fn generate_identifier_map() -> HashMap<String, TokenKind> {
    let mut identifier_map = HashMap::new();
    identifier_map.insert(String::from("and"), TokenKind::And);
    identifier_map.insert(String::from("class"), TokenKind::Class);
    identifier_map.insert(String::from("if"), TokenKind::If);
    identifier_map.insert(String::from("else"), TokenKind::Else);
    identifier_map.insert(String::from("true"), TokenKind::True);
    identifier_map.insert(String::from("false"), TokenKind::False);
    identifier_map.insert(String::from("function"), TokenKind::Function);
    identifier_map.insert(String::from("return"), TokenKind::Return);
    identifier_map.insert(String::from("for"), TokenKind::For);
    identifier_map.insert(String::from("console"), TokenKind::Console);
    identifier_map.insert(String::from("var"), TokenKind::Var);
    identifier_map
}
