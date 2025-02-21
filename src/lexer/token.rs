use std::fmt::Debug;

#[derive(Debug)]
pub enum TokenType{
    Number, // 1 2 3
    Plus, // +
    Minus, // -
    Star, // *
    Slash, // /
    LeftRBracket, // (
    RightRBracket, // )
    LeftCBracket, // {
    RightCBracket, // }
    LeftSBracket, // [
    RightSBracket, // ]
}

#[derive(Debug)]
pub struct Token{
    token_type: TokenType,
    index: u32,
    len: u32,
    line: u32
}

impl Token {
    pub fn new(token_type:TokenType, index: u32, len: u32, line: u32) -> Self{
        Self { token_type, index, len, line }
    }
}
