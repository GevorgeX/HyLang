use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq , Copy)]
pub enum TokenType{
    Integer, // 1 2 3
    True, // true,
    False, // false,

    LeftRBracket, // (
    RightRBracket, // )
    LeftCBracket, // {
    RightCBracket, // }
    LeftSBracket, // [
    RightSBracket, // ]

    Ident, //bla bla
    If, //if
    Else, // else
    Elif, // elif
    While, // while
    Function, // function
    Struct, // struct
    Union, // union
    Var, // var
    Const, // const
    Namespace, // namespace
    Use, // use
    Return, // return
    Break, // break
    Continue, // continue

    Or, // or
    And, // and

    Plus, // +
    PlusEqual, // +=
    Minus, // -
    MinusEqual, // -=
    Star, // *
    StarEqual, // *=
    Slash, // /
    SlashEqual, // /=
    Percent, // %
    PercentEqual, // %=

    Not, // !
    NotEqual, // !=
    Equal, // =
    DoubleEqual, // ==
    Less, // <
    LessEqual, // <=
    Greater, // >
    GreaterEqual, // >=

    DoubleLess, // <<
    DoubleLessEqual, // <<=
    DoubleGreater, // >>
    DoubleGreaterEqual, // >>=
    Tilde, // ~
    TildeEqual, // ~=
    Ampersand, // &
    AmpersandEqual, // &=
    Pipe, // |
    PipeEqual, // |=
    Caret, // ^
    CaretEqual, // ^=

    Dot, // .
    Comma, // ,
    
    DotComma, // ;
    NewLine, // \n
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
