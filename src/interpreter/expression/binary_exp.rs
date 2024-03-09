use std::rc::Rc;

use crate::interpreter::library::{exception::Exception, object::ReferenceToObject, object_utils::arithmetical, Context};

use super::{Expression, OperationType};
#[derive(Clone)]

pub struct BinaryExpImpl{
    left: Box<Expression>,
    right: Box<Expression>,
    op: OperationType
}

impl BinaryExpImpl {
    pub fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        let left = match self.left.evaluate(context.clone()){
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        let right = match self.right.evaluate(context.clone()){
            Ok(o) => o,
            Err(e) => return Err(e),
        };


        match self.op {
            OperationType::Plus => Ok(arithmetical::add(left,right)) ,
            OperationType::Minus => Ok(arithmetical::sub(left,right)) ,
            OperationType::Multi => Ok(arithmetical::mult(left,right)) ,
            OperationType::Divide => Ok(arithmetical::div(left,right)) ,
            OperationType::Remain =>Ok(arithmetical::rem(left,right)),
            _ => panic!("Cant use this operator")
        }
    }

    pub fn new(left: Box<Expression>,right: Box<Expression>,op: OperationType) ->Self{
        Self{left ,right ,op}
    }
}
