use crate::{lexer::token::Token, parser::node::Node};

#[derive(Debug)]
pub struct NumberNode{
    token: Token
}

impl NumberNode {
    pub fn new(token:Token) -> Box<Self> {
        Box::new(Self { token })
    }
}

impl Node for NumberNode {
    fn visit(&self) {
        todo!()
    }
}