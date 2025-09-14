use token::{Token , TokenType};
use crate::errors::lexer_errors::LexerError;
use crate::errors::lexer_errors::LexerError::{InvalidNumber, UnexpectedCharacter};

pub mod token;

pub struct Lexer {
    index: usize,
    line: usize,
    start: usize,
    text: Vec<char>
}

impl Lexer {
    pub fn new () -> Self {
        Lexer{
            index: 0,
            line: 1,
            start: 0,
            text: vec![]
        }
    }

    pub fn parse(&mut self,text:&String) ->Result<Vec<Token>, LexerError >{
        self.text = text.chars().collect();
        self.line = 1;
        self.index = 0;
        let mut res = vec![];

        while let Some(&chr) = self.text.get(self.index){
            match chr {
                ' ' | '\t' | '\r' => {
                    self.index += 1;
                    self.start += 1;
                },
                '\n' => {
                    self.line += 1;
                    self.index += 1;
                    self.start = 0;
                }
                ident if Self::is_allowed_ident(ident) => {
                    let name = self.parse_ident();
                    res.push(name)
                }
                digit if digit.is_ascii_digit() => {
                    let num = self.parse_number()?;
                    res.push(num);
                }
                '+' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::PlusEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Plus, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '-' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::MinusEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Minus, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '*' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::StarEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Star, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '/' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::SlashEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Slash, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '%' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::PercentEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Percent, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '!' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::NotEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Not, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '=' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::DoubleEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Equal, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '<' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::LessEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else if self.text.get(self.index + 1) == Some(&'<') {
                        if self.text.get(self.index + 2) == Some(&'=') {
                            res.push(Token::new(TokenType::DoubleLessEqual, self.index, self.start, 3, self.line));
                            self.index += 3;
                            self.start += 3;
                        } else {
                            res.push(Token::new(TokenType::DoubleLess, self.index, self.start, 2, self.line));
                            self.index += 2;
                            self.start += 2;
                        }
                    } else {
                        res.push(Token::new(TokenType::Less, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '>' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::GreaterEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else if self.text.get(self.index + 1) == Some(&'>') {
                        if self.text.get(self.index + 2) == Some(&'=') {
                            res.push(Token::new(TokenType::DoubleGreaterEqual, self.index, self.start, 3, self.line));
                            self.index += 3;
                            self.start += 3;
                        } else {
                            res.push(Token::new(TokenType::DoubleGreater, self.index, self.start, 2, self.line));
                            self.index += 2;
                            self.start += 2;
                        }
                    } else {
                        res.push(Token::new(TokenType::Greater, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '~' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::TildeEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Tilde, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '&' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::AmpersandEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Ampersand, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '|' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::PipeEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Pipe, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '^' => {
                    if self.text.get(self.index + 1) == Some(&'=') {
                        res.push(Token::new(TokenType::CaretEqual, self.index, self.start, 2, self.line));
                        self.index += 2;
                        self.start += 2;
                    } else {
                        res.push(Token::new(TokenType::Caret, self.index, self.start, 1, self.line));
                        self.index += 1;
                        self.start += 1;
                    }
                }
                '.' => {
                    res.push(Token::new(TokenType::Dot, self.index, self.start, 1, self.line));
                    self.index += 1;
                    self.start += 1;
                }
                ',' => {
                    res.push(Token::new(TokenType::Comma, self.index, self.start, 1, self.line));
                    self.index += 1;
                    self.start += 1;
                }
                '(' => {
                    res.push(Token::new(TokenType::LeftRBracket, self.index, self.start, 1, self.line));
                    self.index += 1;
                    self.start += 1;
                }
                ')' => {
                    res.push(Token::new(TokenType::RightRBracket, self.index, self.start, 1, self.line));
                    self.index += 1;
                    self.start += 1;
                }
                '{' => {
                    res.push(Token::new(TokenType::LeftCBracket, self.index, self.start, 1, self.line));
                    self.index += 1;
                    self.start += 1;
                }
                '}' => {
                    res.push(Token::new(TokenType::RightCBracket, self.index, self.start, 1, self.line));
                    self.index += 1;
                    self.start += 1;
                }
                '[' => {
                    res.push(Token::new(TokenType::LeftSBracket, self.index, self.start, 1, self.line));
                    self.index += 1;
                    self.start += 1;
                }
                ']' => {
                    res.push(Token::new(TokenType::RightSBracket, self.index, self.start, 1, self.line));
                    self.index += 1;
                    self.start += 1;
                }
                _ => return Err(UnexpectedCharacter {line: self.line, index: self.index})
            }
        }
        Ok(res)
    }
    
    fn parse_number(&mut self) -> Result<Token, LexerError> {
        let start = self.index;
        while let Some(&chr) = self.text.get(self.index) {
            if chr.is_ascii_digit(){
                self.index += 1;
            }
            else{
                break;
            }
        }

        if let Some(&chr) = self.text.get(self.index) {
            if Self::is_allowed_ident(chr){
                return Err(InvalidNumber{line: self.line,index: start, len: self.index - start});
            }
        }

        Ok(Token::new(TokenType::Integer, start, self.start, self.index - start, self.line))
    }

    fn parse_ident(&mut self) -> Token {
        let start = self.index;
        while let Some(&chr) = self.text.get(self.index) {
            if Self::is_allowed_ident(chr) || chr.is_ascii_digit(){
                self.index += 1;
            }
            else{
                break;
            }
        }

        let word:String = self.text[start..self.index].iter().collect();
        let token_type = match &*word {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "elif" => TokenType::Elif,
            "while" => TokenType::While,
            "fn" => TokenType::Function,
            "struct" => TokenType::Struct,
            "union" => TokenType::Union,
            "var" => TokenType::Var,
            "const" => TokenType::Const,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "namespace" => TokenType::Namespace,
            "use" => TokenType::Use,
            "return" => TokenType::Return,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _=> TokenType::Ident
        };

        Token::new(token_type , start, self.start, self.index - start, self.line)
    }

    fn is_allowed_ident(chr:char) -> bool{
        chr.is_ascii_alphabetic() ||
            chr == '_' ||
            chr as u32 > 127
    }
}
