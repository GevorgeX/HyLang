mod lexer;
mod errors;

fn main() {
    let code = "1 + 3 if *(10/999) - 8barev".to_string();
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
