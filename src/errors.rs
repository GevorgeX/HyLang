pub mod lexer_errors{
    #[derive(Debug)]
    pub enum LexerError{
        UnexpectedCharacter{line: usize, index: usize},
        InvalidNumber{line: usize, index: usize, len: usize},
    }
}