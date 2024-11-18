use std::collections::HashMap;

use super::{
    report::Report,
    token::{generate_identifier_map, TextSpan, Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scanner<'a> {
    source: &'a str,
    cur_pos: usize,
    start: usize,
    line: u32,
    tokens: Vec<Token>,
    keywords_map: HashMap<String, TokenKind>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let keywords_map = generate_identifier_map();
        Scanner {
            source,
            tokens: Vec::new(),
            cur_pos: 0,
            start: 0,
            line: 1,
            keywords_map,
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !(self.is_at_end()) {
            self.start = self.cur_pos;
            if self.is_number_start() {
                self.consume_number();
            } else if self.is_string_start(None) {
                self.consume_string();
            } else if self.is_alpha() {
                self.consume_identifier();
            } else {
                self.consume_punctuation();
            }
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

    fn peek_offset(&mut self, offset: u32) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.cur_pos + offset as usize)
            .unwrap()
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
        let next_c = self.peek_offset(1);
        if next_c == expected {
            self.consume();
            return true;
        }
        false
    }

    fn is_alpha(&mut self) -> bool {
        let c = self.current_char().unwrap();
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
    }

    fn is_alpha_numeric(&mut self, c: char) -> bool {
        self.is_alpha() || c.is_digit(10)
    }

    fn consume_identifier(&mut self) {
        let c = self.peek();
        while self.is_alpha_numeric(c) {
            self.consume();
        }
        let literal = self.source[self.start..self.cur_pos].to_string();
        let cur_type = self.keywords_map.get(&literal);
        match cur_type {
            Some(c) => self.add_token(c.clone(), Some(literal)),
            None => self.add_token(TokenKind::Identifier, Some(literal)),
        }
    }

    fn is_string_start(&mut self, c: Option<char>) -> bool {
        let cc = if c.is_some() {
            c.unwrap()
        } else {
            self.current_char().unwrap()
        };
        cc == '"'
    }

    fn is_number_start(&mut self) -> bool {
        self.current_char().unwrap().is_digit(10)
    }
    fn consume_string(&mut self) {
        // * skip first \"\
        self.consume();
        // println!("what is cur{:?}", self.peek());
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.consume();
        }
        if self.is_at_end() {
            Report::new(self.line, "", "Unterminated string.");
        }
        self.consume();
        let literal = self.source[self.start + 1..self.cur_pos - 1].to_string();
        self.add_token(TokenKind::String, Some(literal));
    }

    // TODO decimals
    fn consume_number(&mut self) {
        let mut number = 0;

        while let Some(n) = self.current_char() {
            if n.is_digit(10) {
                self.consume();
                number = number * 10 + n.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        self.add_token(TokenKind::Number(number), None);
    }
    fn consume_punctuation(&mut self) {
        let c = self.consume().unwrap();
        let mut kind = TokenKind::Bad;
        match c {
            '(' => {
                kind = TokenKind::LeftParen;
            }
            ')' => {
                kind = TokenKind::RightParen;
            }
            '{' => {
                kind = TokenKind::LeftBrace;
            }
            '}' => {
                kind = TokenKind::RightBrace;
            }
            ',' => {
                kind = TokenKind::Comma;
            }
            '.' => {
                kind = TokenKind::Dot;
            }
            '-' => {
                kind = TokenKind::Minus;
            }
            '+' => {
                kind = TokenKind::Plus;
            }
            '*' => {
                kind = TokenKind::Star;
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.consume();
                    }
                } else {
                    kind = TokenKind::Slash;
                }
            }
            ';' => {
                kind = TokenKind::Semicolon;
            }
            '!' => {
                kind = if self.is_match('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
            }
            '=' => {
                kind = if self.is_match('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
            }
            '<' => {
                kind = if self.is_match('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
            }
            '>' => {
                kind = if self.is_match('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
            }
            '\r' | ' ' | '\t' => {
                kind = TokenKind::Whitespace;
            }
            '\n' => {
                self.line += 1;
                return;
            }
            _ => {
                Report::new(self.line, &c.to_string(), "Unexpected character.");
            }
        }
        self.add_token(kind, None);
    }

    /**
     * literal_props => default value by set Option<>, a bit weird
     */
    fn add_token(&mut self, kind: TokenKind, literal_props: Option<String>) {
        let literal = if literal_props.is_some() {
            literal_props.unwrap()
        } else {
            self.source[self.start..self.cur_pos].to_string()
        };
        let token = Token::new(
            kind,
            self.line,
            TextSpan::new(self.start, self.cur_pos, literal),
        );
        self.tokens.push(token);
    }
}
