pub mod lexer_errors{
    #[derive(Debug)]
    pub enum LexerError{
        UnexpectedCharacter{line: usize, index: usize},
        InvalidNumber{line: usize, index: usize, len: usize},
    }
}

pub mod syntax_errors{
    use crate::lexer::token::{Token, TokenInfo, TokenType};

    #[derive(Debug)]
    pub enum SyntaxError{
        UnexpectedToken{token:Token},
        ExpectedToken{token_info: TokenInfo, expected: TokenType},
        ExpectedExpression{token_info: TokenInfo},
        ExpectedStatement{token_info: TokenInfo},
    }
}