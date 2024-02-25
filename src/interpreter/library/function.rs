use crate::interpreter::statement::Statement;

use super::{local_context::LocalContext, Context, ReferenceToObject};

// #[derive(Clone)]
pub struct Function<'a>{
    context: &'a Context<'a>,
    args: Vec<String>,
    body: Box<dyn Statement>
}

impl<'a> Function<'a> {
    pub fn call(&self, arguments: Vec<ReferenceToObject>) {
        let  cur_context = LocalContext::new(Some(self.context));

        if self.args.len() != arguments.len() {
            panic!("ARGUMENT ERROR");
        }

        for i in 0..(self.args.len()) {
            cur_context.define_variable(self.args[i].clone(),arguments[i].clone());
        }

        let cur_context = Context::LocalContext(cur_context);
        self.body.interpret(&cur_context);

    }

    pub fn new(context:&'a Context<'a>, args: Vec<String> , body: Box<dyn Statement> ) -> Function<'a>{
        Function { context, args, body}
    }
}

