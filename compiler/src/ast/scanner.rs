use super::report::Report;

// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scanner<'a> {
    source: &'a str,
    cur_pos: usize,
    start: usize,
    line: u32,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            cur_pos: 0,
            start: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !(self.is_at_end()) {
            println!("????");
            self.start = self.cur_pos;
            self.consume_punctuation();
        }
        let eof_char = '\0';
        self.tokens.push(Token::new(
            TokenKind::Eof,
            self.line,
            TextSpan::new(0, 0, eof_char.to_string()),
        ));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.cur_pos >= self.source.len()
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.current_char().unwrap()
    }

    fn current_char(&mut self) -> Option<char> {
        self.source.chars().nth(self.cur_pos)
    }

    // advance
    fn consume(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let c = self.current_char();
        self.cur_pos += 1;
        c
    }

    fn is_match(&mut self, expected: char) -> bool {
        println!("{expected}");
        self.consume().unwrap() == expected
    }

    fn consume_punctuation(&mut self) {
        let c = self.consume().unwrap();
        match c {
            '(' => {
                self.add_token(TokenKind::LeftParen);
            }
            ')' => {
                self.add_token(TokenKind::RightParen);
            }
            '{' => {
                self.add_token(TokenKind::LeftBrace);
            }
            '}' => {
                self.add_token(TokenKind::RightBrace);
            }
            ',' => {
                self.add_token(TokenKind::Comma);
            }
            '.' => {
                self.add_token(TokenKind::Dot);
            }
            '-' => {
                self.add_token(TokenKind::Minus);
            }
            '+' => {
                self.add_token(TokenKind::Plus);
            }
            '*' => {
                self.add_token(TokenKind::Star);
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.consume();
                    }
                } else {
                    self.add_token(TokenKind::Slash);
                }
            }
            ';' => {
                self.add_token(TokenKind::Semicolon);
            }
            '!' => {
                let k = if self.is_match('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add_token(k);
            }
            '=' => {
                let k = if self.is_match('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(k);
            }
            '<' => {
                let k = if self.is_match('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add_token(k);
            }
            '>' => {
                let k = if self.is_match('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add_token(k);
            }
            '\r' | ' ' | '\t' => (),
            '\n' => {
                self.line += 1;
            }
            _ => {
                self.add_token(TokenKind::Semicolon);
                Report::new(self.line, "", "Unexpected character.");
            }
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        let literal = self.source[self.start..self.cur_pos].to_string();
        let token = Token::new(
            kind,
            self.line,
            TextSpan::new(self.start, self.cur_pos, literal),
        );
        self.tokens.push(token);
    }
}

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
    Else,
    False,
    Function,
    For,
    If,
    Or,
    Console,
    Return,
    Var,
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
