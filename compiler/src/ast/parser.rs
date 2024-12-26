use super::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    cur_pos: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cur_pos: 0 }
    }

    // TODO
    fn equality() {}

    fn is_match(&mut self, kinds: Vec<TokenKind>) -> bool {
        for kind in kinds {
            if self.check(kind) == true {
                self.consume();
                return true;
            }
        }
        return false;
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cur_pos)
    }

    fn is_at_end(&mut self) -> bool {
            self.peek().unwrap().kind == TokenKind::Eof
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.cur_pos - 1)
    }

    fn consume(&mut self)-> Option<&Token> {
        if !(self.is_at_end()) {
            self.cur_pos += 1;
        }
        self.previous()
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            return self.peek().unwrap().kind == kind;
        }
    }

}
