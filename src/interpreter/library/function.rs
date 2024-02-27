use std::rc::Rc;

use crate::interpreter::{expression::Expression, statement::Statement};

use super::{local_context::LocalContext, Context};

#[derive(Clone)]
pub struct Function{
    define_context: Rc<Context>,
    args: Vec<String>,
    body: Box<dyn Statement>
}



impl Function {
    pub fn call(&self, arguments: Vec<Box<dyn Expression>> , context: Rc<Context> ) {
        let cur_context = LocalContext::new(Some(Rc::downgrade(&self.define_context)));

        if self.args.len() != arguments.len() {
            panic!("ARGUMENT ERROR");
        }

        for i in 0..(self.args.len()) {
            cur_context.define_variable(self.args[i].clone(),arguments[i].evaluate(context.clone()));
        }

        let cur_context = Context::LocalContext(cur_context);
        self.body.interpret(Rc::new(cur_context));

    }

    pub fn new(define_context:Rc<Context>, args: Vec<String> , body: Box<dyn Statement> ) -> Function{
        Function { define_context, args, body}
    }
}

