#[derive(Debug ,PartialEq, Eq)]
pub enum Token{
    Word(String),
    Number(i32),
    TrueFalse(bool),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Equal,
    DoubleEqual,
    EOF,
    Define,
    IF,
    ELSE,
    And,
    Or,
    More,
    MoreEqual,
    Less,
    LessEqual,
    Not,
    NotEqual,
    While,
    Break,
    Continue,
    Apostr,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    Function,
    Return,
    Percent,

    P_R_I_N_T,
    LeftSquareBrace,
    RightSquareBrace,
    Comma,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}