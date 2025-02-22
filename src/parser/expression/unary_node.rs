use crate::{lexer::token::Token, parser::node::{DNode, Node}};

#[derive(Debug)]
pub struct UnaryNode{
    token: Token,
    right: DNode,
    op: UnaryOp
}

impl UnaryNode {
    pub fn new(token:Token,right:DNode, op:UnaryOp ) -> Box<Self> {
        Box::new(Self { token, right, op })
    }
}

impl Node for UnaryNode {
    fn visit(&self) {
        todo!()
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
    Not
}