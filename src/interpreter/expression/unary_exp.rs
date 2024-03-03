use std::rc::Rc;

use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::object::ReferenceToObject;
use crate::interpreter::library::object_utils::{arithmetical::neg, logical::not};
use crate::interpreter::library::Context;

use super::{Expression, OperationType};

#[derive(Clone)]
pub struct UnaryExp{
    value: Box<dyn Expression>,
    op: OperationType
}

impl super::Expression for UnaryExp {
    fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject, Exception> {
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
}

impl UnaryExp {
    pub fn new(value:Box<dyn Expression>, op:OperationType) -> UnaryExp {
        UnaryExp{value , op}
    }
    // pub fn to_string(&self)->String {
    //     format!("UnaryExp<{}, {}>",self.op.to_string() ,self.value)
    // }
}

