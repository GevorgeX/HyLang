use crate::debug::print_expression_tree;

mod lexer;
mod errors;
mod parser;
mod debug;

fn main() {
    let code = std::fs::read_to_string("code.hy").unwrap();
    let mut lex = lexer::Lexer::new();

    let res = lex.parse(&code);

    match &res {
        Ok(res) => {
            for i in res.iter() {
                println!("{:?}", i);
            }
        }
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    }

    let mut parser = parser::Parser::new(res.unwrap());
    let res = parser.parse();
    match res {
        Ok(res) => {
            print_expression_tree(&res, 0);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

}
