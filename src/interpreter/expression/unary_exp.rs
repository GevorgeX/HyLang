use std::rc::Rc;

use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::object::ReferenceToObject;
use crate::interpreter::library::object_utils::{arithmetical::neg, logical::not};
use crate::interpreter::library::Context;

use super::{Expression, OperationType};
#[derive(Clone)]

pub struct UnaryExpImpl{
    pub value: Box<Expression>,
    pub op: OperationType
}

impl UnaryExpImpl {
    pub fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject, Exception> {
        match self.op {
            OperationType::Plus => self.value.evaluate(context),
            OperationType::Minus =>
                match self.value.evaluate(context){
                    Ok(o) => Ok(neg(o)),
                    Err(e) => return Err(e),
                },
            OperationType::Not =>
            match self.value.evaluate(context){
                Ok(o) => Ok(not(o)),
                Err(e) => return Err(e),
            },
            _ => panic!("wrong operator")
        }
    }
    pub fn new(value: Box<Expression>,op: OperationType) ->Self{
        Self{value ,op}
    }
}