pub enum TokenKind {
    Number,
    Plus,
    Minus,
    Asterisk,
    Slash,
    EndOfFile,
    Bad,
    Whitespace,
    LeftParen,
    RightParen,
}

pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}
impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self { start,  end,  literal }
    }
}

pub struct  Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
}
impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Token { kind, span}
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}
impl<'a>  Lexer<'a> {
    pub fn new(input: &'a str) -> Self{
        Lexer { input, current_pos: 0 }
    }

    // pub fn next_token() -> Option<Token> {}

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn current_char(&self) -> Option<char>{
        self.input.chars().nth(self.current_pos)
    }

    fn consume() {}
}