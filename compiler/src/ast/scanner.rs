// TODO
#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<String>
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner { source, tokens: Vec::new() }
    }
    pub fn scan_tokens() {}
}

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
    Else,
    False,
    Function,
    For,
    If,
    // TODO
}