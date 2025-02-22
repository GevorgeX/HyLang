use std::fmt::Debug;

#[derive(Debug, PartialEq)]
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
    Word, //bla bla
    If,
    Else,
    While,
}

#[derive(Debug)]
pub struct Token{
    pub token_type: TokenType,
    index: u32,
    len: u32,
    line: u32
}

impl Token {
    pub fn new(token_type:TokenType, index: u32, len: u32, line: u32) -> Self{
        Self { token_type, index, len, line }
    }
}
