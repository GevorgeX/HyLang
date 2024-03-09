use std::rc::Rc;

use crate::interpreter::{expression::Expression, statement::Statement};

use super::{exception::Exception, local_context::LocalContext, object::{create_object, ReferenceToObject}, Context};

#[derive(Clone)]
pub struct Function{
    define_context: Rc<Context>,
    args: Vec<String>,
    body: Box<dyn Statement>
}



impl Function {
    pub fn call(&self, arguments: Vec<Box<Expression>> , context: Rc<Context>)-> Result<ReferenceToObject, Exception> {
        let cur_context = LocalContext::new(Some(Rc::downgrade(&self.define_context)));

        if self.args.len() != arguments.len() {
            panic!("ARGUMENT ERROR");
        }

        for i in 0..(self.args.len()) {
            let refer = match arguments[i].evaluate(context.clone()){
                Ok(o) => o,
                Err(e) => return Err(e),
            };
            match cur_context.define_variable(self.args[i].clone(),refer) {
                Ok(_) => (),
                Err(e) => return Err(e),
            } 
        }

        let cur_context = Context::LocalContext(cur_context);
        let body = self.body.interpret(Rc::new(cur_context));
        
        match body {
            Ok(tsk) => match tsk {
                crate::interpreter::task::Task::Return(val) => Ok(val),
                _ => Ok(create_object(super::object::Object::Null))
            },
            Err(e) => Err(e),
        }
    }

    pub fn new(define_context:Rc<Context>, args: Vec<String> , body: Box<dyn Statement> ) -> Function{
        Function { define_context, args, body}
    }
}

