use token::{Token , TokenType};
use crate::errors::lexer_errors::LexerError;
use crate::errors::lexer_errors::LexerError::{InvalidNumber, UnexpectedCharacter};

mod token;

pub struct Lexer {
    index: usize,
    line: usize,
    text: Vec<char>
}

impl Lexer {
    pub fn new () -> Self {
        Lexer{
            index: 0,
            line: 1,
            text: vec![]
        }
    }

    pub fn parse(&mut self,text:&String) ->Result<Vec<Token>, LexerError >{
        self.text = text.chars().collect();
        self.line = 1;
        self.index = 0;

        let mut res = vec![];

        while let Some(&chr) = self.text.get(self.index){
            self.skip_space();

            if chr == '\n'{
                self.line += 1;
                self.index += 1;
            }
            else if is_allowed_word(chr){
                let name = self.parse_word();
                res.push(name)
            }
            else if chr.is_ascii_digit(){
                let num = self.parse_number()?;
                res.push(num);
            }
            else if chr == '+'{
                res.push(Token::new(TokenType::Plus, self.index, 1, self.line));
                self.index += 1;
            }
            else if chr == '-'{
                res.push(Token::new(TokenType::Minus, self.index, 1, self.line));
                self.index += 1;
            }
            else if chr == '*'{
                res.push(Token::new(TokenType::Star, self.index, 1, self.line));
                self.index += 1;
            }
            else if chr == '/'{
                res.push(Token::new(TokenType::Slash, self.index, 1, self.line));
                self.index += 1;
            }
            else if chr == '('{
                res.push(Token::new(TokenType::LeftRBracket, self.index, 1, self.line));
                self.index += 1;
            }
            else if chr == ')'{
                res.push(Token::new(TokenType::RightRBracket, self.index, 1, self.line));
                self.index += 1;
            }
            else if chr == '{'{
                res.push(Token::new(TokenType::LeftCBracket, self.index, 1, self.line));
                self.index += 1;
            }
            else if chr == '}'{
                res.push(Token::new(TokenType::RightCBracket, self.index, 1, self.line));
                self.index += 1;
            }
            else if chr == '['{
                res.push(Token::new(TokenType::LeftSBracket, self.index, 1, self.line));
                self.index += 1;
            }
            else if chr == ']'{
                res.push(Token::new(TokenType::RightSBracket, self.index, 1, self.line));
                self.index += 1;
            }
            else{
                return Err(UnexpectedCharacter {line: self.line, index: self.index});
            }
        }
        Ok(res)
    }

    fn skip_space(&mut self) {
        while let Some(chr) = self.text.get(self.index) {
            match chr {
                ' ' | '\t' | '\r' => self.index +=1,
                _ => return
            }
        }
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
            if is_allowed_word(chr){
                return Err(InvalidNumber{line: self.line,index: start, len: self.index - start});
            }
        }

        Ok(Token::new(TokenType::Number, start, self.index - start, self.line))
    }

    fn parse_word(&mut self) -> Token {
        let start = self.index;
        while let Some(&chr) = self.text.get(self.index) {
            if is_allowed_word(chr) || chr.is_ascii_digit(){
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
            "while" => TokenType::While,
            _=> TokenType::Word
        };

        Token::new(token_type , start, self.index - start, self.line)
    }
    
}

fn is_allowed_word(chr:char) -> bool{
    chr.is_ascii_alphabetic() || 
    chr == '_' || 
    chr as u32 > 127 
}