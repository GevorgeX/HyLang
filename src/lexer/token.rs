use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq , Copy)]
pub enum TokenType{
    Integer, // 1 2 3
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
    Elif,
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
    NotEqual,
    Less,
    Greater,
    Comma,
    Tilde,
    // Colon,
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

#[derive(Debug, Clone, Copy)]
pub struct TokenInfo {
    pub index: usize,
    pub start: usize,
    pub len: usize,
    pub line: usize
}

#[derive(Debug, Clone, Copy)]
pub struct Token{
    pub token_type: TokenType,
    pub token_info: TokenInfo
}

impl Token {
    pub fn new(token_type:TokenType, index: usize, start: usize, len: usize, line: usize) -> Self{
        Self { token_type,token_info: TokenInfo { index, start, len, line } }
    }
}
