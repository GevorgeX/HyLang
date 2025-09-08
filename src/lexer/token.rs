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
    Word, //bla bla
    If,
    Else,
    While,
}

#[derive(Debug)]
pub struct Token{
    token_type: TokenType,
    index: usize,
    len: usize,
    line: usize
}

impl Token {
    pub fn new(token_type:TokenType, index: usize, len: usize, line: usize) -> Self{
        Self { token_type, index, len, line }
    }
}
