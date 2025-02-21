mod lexer;

fn main() {
    let code = "1 + 3 *(10/999) - 8 barev".to_string();
    let mut lex = lexer::Lexer::new();

    let res = lex.parse(&code);

    for i in res.iter() {
        println!("{:?}", i);
    }
}
