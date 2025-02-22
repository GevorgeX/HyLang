use node::Node;

use crate::lexer::token::{Token, TokenType};

mod expression;
mod node;

struct Parser{
    index: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { index: 0, tokens }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Box<dyn Node>{
        self.index = 0;
        self.tokens = tokens;

        todo!()
    }

    fn get(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn next(&mut self) {
        self.index += 1
    }

    fn match_type(&mut self, token_type: TokenType) -> bool {
        if self.get().token_type == token_type{
            self.next();
            true
        }
        else{
            false
        }
    }
}