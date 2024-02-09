pub mod token;

use token::Token;

const DELIMITERS: [char;14]= ['+' , '-' ,'*', '/', '(',')' ,'=', '>' ,'<','!' ,'{','}', '\'','\"' ];

pub fn parse(chars: &String) -> Vec<Token>{
    let mut res = vec![];

    let mut index = 0;
    let chars:Vec<char> = chars.chars().collect();
    let len = chars.len();

    while index < len {
        let chr = chars[index];
        
        if chr.is_numeric(){
            res.push(get_number_token(&chars,len , &mut index) );
        }
        else if chr.is_alphabetic() || chr == '_'  {            
            res.push(get_text_token(&chars,len , &mut index));        
        }
        else if DELIMITERS.contains(&chr){
            res.push(get_operator_token(&chars, &mut index));
            match chars[index -1] {
              '\'' =>{
                    res.push(get_char_literal(&chars,len , &mut index));
                    if chars[index] == '\''{
                        res.push(Token::Apostr);
                        index += 1
                    }  
                },
              _=> ()
            };
        }
        else{
            index += 1;
        }
        
    }
    res
}

fn get_number_token(chars: &Vec<char> ,len:usize, index: &mut usize )->Token {
    let mut res = String::new();

    while  *index < len && chars[*index].is_numeric()  {
        res.push(chars[*index]);

        *index += 1;
    }
    
    Token::Number(
        res.parse::<i32>().unwrap() 
    )
}
fn get_text_token(chars: &Vec<char> ,len:usize, index: &mut usize )->Token {
    let mut res = chars[*index].to_string();
    *index += 1;

    while  *index < len    {
        let next_char = chars[*index];

        if (next_char.is_alphanumeric() || next_char == '_' ) && !DELIMITERS.contains(&next_char) && next_char != ' ' && next_char != '\n'{
            res.push(chars[*index]);
            *index += 1;
        }
        else {
            break;
        }

    }
    
    match res.as_str() {
        "հայ" => Token::Define,
        "ճիշտ" => Token::TrueFalse(true),
        "կեխծ" => Token::TrueFalse(false),
        "եթե" => Token::IF,
        "ապա" => Token::ELSE,
        "և" => Token::And,
        "կամ" => Token::Or,
        "ցիկլ" => Token::While,
        "կայնի" => Token::Break,
        "շարունակի" => Token::Continue,
        "տպի" => Token::P_R_I_N_T,
        _=> Token::Word(res)
    }
}
fn get_operator_token(chars: &Vec<char>, index: &mut usize)-> Token {
    let chr = chars[*index];
    *index += 1;
    
    match chr {
        '+' => {
            if chars[*index ] == '='{
                *index += 1;
                return Token::PlusEqual
            }
            else{
                return Token::Plus
            }
        }, 
        '-' => {
            if chars[*index ] == '='{
                *index += 1;
                return Token::MinusEqual
            }
            else{
                return Token::Minus
            }
        },
        '*' => {
            if chars[*index ] == '='{
                *index += 1;
                return Token::StarEqual
            }
            else{
                return Token::Star
            }
        }, 
        '/' => {
            if chars[*index ] == '='{
                *index += 1;
                return Token::SlashEqual
            }
            else{
                return Token::Slash
            }
        },
        '(' => Token::LeftParenthesis,
        ')' => Token::RightParenthesis ,
        '=' => {
            if chars[*index ] == '='{
                *index += 1;
                return Token::DoubleEqual
            }
            else{
                return Token::Equal
            }
        },
        '!' =>{
            if chars[*index ] == '='{
                *index += 1;
                return Token::NotEqual
            }
            else{
                return Token::Not
            }
        }
        '>' => {
            if chars[*index ] == '='{
                *index += 1;
                return Token::MoreEqual
            }
            else{
                return Token::More
            }
        },
        '<' => {
            if chars[*index ] == '='{
                *index += 1;
                return Token::LessEqual
            }
            else{
                return Token::Less
            }
        },
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        '\''=> Token::Apostr,
        _ => panic!("Undefined Token")
    }
}
fn get_char_literal(chars: &Vec<char>,len:usize, index: &mut usize) -> Token {
    let mut t = String::new();
    while  *index + 1 < len {
        let  chr = chars[*index];
        if chr != '\''{
            t.push(chr);
            *index += 1;
        }
        else{
            break;
        }
    }
    Token::Word(t)
}

pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}