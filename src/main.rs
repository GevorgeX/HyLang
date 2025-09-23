use crate::debug::{print_declaration_tree, print_statement_tree};
use crate::parser::declaration::Declaration;

mod lexer;
mod errors;
mod parser;
mod debug;

fn main() {
    let code = std::fs::read_to_string("code.hy").unwrap();
    let mut lex = lexer::Lexer::new();

    let res = lex.parse(&code);

    // match &res {
    //     Ok(res) => {
    //         for i in res.iter() {
    //             println!("{:?}", i);
    //         }
    //     }
    //     Err(err) => {
    //         println!("{:?}", err);
    //         return;
    //     }
    // }

    let mut parser = parser::Parser::new(res.unwrap());
    let res = parser.parse();
    match res {
        Ok(res) => {
            match res {
                Some(res) => {
                    print_declaration_tree(&res, 0, false, false);
                }
                None => {}
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

}
