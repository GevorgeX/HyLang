use crate::{lexer::token::Token, parser::node::{DNode, Node}};

#[derive(Debug)]
pub struct BinaryNode {
    token: Token,
    left: DNode,
    right: DNode,
    op: BinaryOp,
}

impl BinaryNode {
    pub fn new(token: Token, left: DNode, right: DNode, op: BinaryOp) -> Box<Self> {
        Box::new(Self { token, left, right, op })
    }
}

impl Node for BinaryNode {
    fn visit(&self) {
        todo!()
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mult,
    Dev,
}
