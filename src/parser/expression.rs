use number_node::NumberNode;

use super::{node::DNode, Parser};

mod number_node;
mod word_node;
mod unary_node;
mod binary_node;

impl Parser {
    fn primary(&self) -> Result<DNode,()> {
        todo!()
    }
}