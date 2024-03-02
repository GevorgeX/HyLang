use std::rc::Rc;

use crate::interpreter::library::{exception::Exception, object::ReferenceToObject, object_utils::arithmetical, Context};

use super::{Expression, OperationType};

#[derive(Clone)]
pub struct BinaryExp{
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    op: OperationType
}

impl super::Expression for BinaryExp {
    fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        let left =self.left.evaluate(context.clone()).unwrap();
        let right = self.right.evaluate(context.clone()).unwrap();
        match self.op {
            OperationType::Plus => Ok(arithmetical::add(left,right)) ,
            OperationType::Minus => Ok(arithmetical::sub(left,right)) ,
            OperationType::Multi => Ok(arithmetical::mult(left,right)) ,
            OperationType::Divide => Ok(arithmetical::div(left,right)) ,
            _ => panic!("Cant use this operator")
        }
    }
}

impl BinaryExp {
    pub fn new(left:Box<dyn Expression> , right:Box<dyn Expression>, op:OperationType) -> BinaryExp {
        BinaryExp{left, right , op}
    }
}

