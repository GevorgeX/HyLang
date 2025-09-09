mod lexer;
mod errors;

fn main() {
    let code = std::fs::read_to_string("code.hy").unwrap();
    let mut lex = lexer::Lexer::new();

    let res = lex.parse(&code);

    match res {
        Ok(res) => {
            for i in res.iter() {
                println!("{:?}", i);
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

}
