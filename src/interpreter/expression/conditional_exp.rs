use std::rc::Rc;

use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::object_utils::{conditional, logical};
use crate::interpreter::library::object::ReferenceToObject;
use crate::interpreter::library::Context;

use super::{Expression, OperationType};

#[derive(Clone)]
pub struct ConditionalExp{
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    op: OperationType
}

impl super::Expression for ConditionalExp {
    fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        let left = self.left.evaluate(context.clone()).unwrap();
        let right =self.right.evaluate(context.clone()).unwrap(); 
        match self.op {
            OperationType::And => Ok(logical::and(left,right)),
            OperationType::Or =>Ok(logical::or(left,right)),
            OperationType::More =>Ok(conditional::more(left,right))  ,
            OperationType::Less =>Ok(conditional::less(left,right))  ,
            OperationType::MoreOrEq =>Ok(conditional::more_or_eq(left,right))  ,
            OperationType::LessOrEq =>Ok(conditional::less_or_eq(left,right))  ,
            OperationType::DoubleEqual => Ok(conditional::equal(left,right))  ,
            OperationType::NotEqual => Ok(conditional::not_equal(left,right))  ,
            _ => panic!("Cant use this operator")
        }
    }
}

impl ConditionalExp {
    pub fn new(left:Box<dyn Expression> , right:Box<dyn Expression>, op:OperationType) -> ConditionalExp {
        ConditionalExp{left, right , op}
    }
}

