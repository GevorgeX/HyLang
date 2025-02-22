use crate::{lexer::token::Token, parser::node::Node};

#[derive(Debug)]
pub struct WordNode{
    token: Token
}

impl WordNode {
    pub fn new(token:Token) -> Box<Self> {
        Box::new(Self { token })
    }
}
impl Node for WordNode {
    fn visit(&self) {
        todo!()
    }
}