mod lexer;
mod interpreter;

fn main() {
    let code= lexer::read_file("code.hylang");

    let parsed = lexer::parse(&code);

    // for i in &parsed {
    //     println!("{}", i);
    // }

    let inter = interpreter::Interpreter::new(parsed);

    inter.run();

    // for  (name , val) in inter.references.borrow().iter(){
    //     println!("{} refcount {} , val = {}",name, std::rc::Rc::strong_count(val) ,val.to_string() );
    // }
}
