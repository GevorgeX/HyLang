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
    Ident, //bla bla
    If,
    Else,
    While,
    Function,
    Struct,
    Union,
    Var,
    Const,
    Or,
    And,
    Not,
    Percent,
    Equal,
    Less,
    Greater,
    Comma,
    Tilde,
    Colon,
    Ampersand,
    Pipe,
    Caret,
    Dot,
    Namespace,
    Use,
    Return,
    Break,
    Continue,
    True,
    False,
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
