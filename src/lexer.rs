use token::{Token , TokenType};

mod token;

pub struct Lexer {
    index: usize,
    line: u32,
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

    pub fn parse(&mut self,text:&String) -> Vec<Token>{
        self.text = text.chars().collect();
        self.line = 1;
        self.index = 0
        ;
        let mut res = vec![];

        while self.not_end(){
            self.skip_space();

            let chr = self.text[self.index];

            if chr == '\n'{
                self.line += 1;
                self.index += 1;
            }
            else if chr.is_ascii_digit(){
                let num = self.parse_number();
                res.push(num);
            }
            else if chr == '+'{
                res.push(Token::new(TokenType::Plus, self.index as u32, 1, self.line));
                self.index += 1;
            }
            else if chr == '-'{
                res.push(Token::new(TokenType::Minus, self.index as u32, 1, self.line));
                self.index += 1;
            }
            else if chr == '*'{
                res.push(Token::new(TokenType::Star, self.index as u32, 1, self.line));
                self.index += 1;
            }
            else if chr == '/'{
                res.push(Token::new(TokenType::Slash, self.index as u32, 1, self.line));
                self.index += 1;
            }
            else if chr == '('{
                res.push(Token::new(TokenType::LeftRBracket, self.index as u32, 1, self.line));
                self.index += 1;
            }
            else if chr == ')'{
                res.push(Token::new(TokenType::RightRBracket, self.index as u32, 1, self.line));
                self.index += 1;
            }
            else if chr == '{'{
                res.push(Token::new(TokenType::LeftCBracket, self.index as u32, 1, self.line));
                self.index += 1;
            }
            else if chr == '}'{
                res.push(Token::new(TokenType::RightCBracket, self.index as u32, 1, self.line));
                self.index += 1;
            }
            else if chr == '['{
                res.push(Token::new(TokenType::LeftSBracket, self.index as u32, 1, self.line));
                self.index += 1;
            }
            else if chr == ']'{
                res.push(Token::new(TokenType::RightRBracket, self.index as u32, 1, self.line));
                self.index += 1;
            }
        }
        res
    }

    fn not_end(&self) ->bool{
        self.index < self.text.len()
    }

    fn skip_space(&mut self) {
        loop {
            let chr = self.text[self.index];
            match chr {
                ' ' | '\t' => self.index +=1,
                _ => return
            }
        }
        
    }
    
    fn parse_number(&mut self) -> Token {
        let start = self.index;
        while self.not_end() && self.text[self.index].is_ascii_digit() {
            self.index += 1;
        }

        Token::new(TokenType::Number , start as u32, (self.index - start) as u32, self.line)
    }
}