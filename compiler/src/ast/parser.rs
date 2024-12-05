use super::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    cur_pos: u32,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cur_pos: 0 }
    }

    // TODO
    fn equality() {}
}
