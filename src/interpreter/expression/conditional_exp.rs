use std::rc::Rc;

use crate::interpreter::library::object_utils::{conditional, logical};
use crate::interpreter::library::{ Context, ReferenceToObject};

use super::{Expression, OperationType};

#[derive(Clone)]
pub struct ConditionalExp{
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    op: OperationType
}

impl super::Expression for ConditionalExp {
    fn evaluate(&self,context:Rc<Context>) -> ReferenceToObject {
        match self.op {
            OperationType::And => logical::and(self.left.evaluate(context.clone()), self.right.evaluate(context.clone()))  ,
            OperationType::Or =>logical::or(self.left.evaluate(context.clone()), self.right.evaluate(context.clone()))  ,
            OperationType::More =>conditional::more(self.left.evaluate(context.clone()), self.right.evaluate(context.clone()))  ,
            OperationType::Less =>conditional::less(self.left.evaluate(context.clone()), self.right.evaluate(context.clone()))  ,
            OperationType::MoreOrEq =>conditional::more_or_eq(self.left.evaluate(context.clone()), self.right.evaluate(context.clone()))  ,
            OperationType::LessOrEq =>conditional::less_or_eq(self.left.evaluate(context.clone()), self.right.evaluate(context.clone()))  ,
            OperationType::DoubleEqual => conditional::equal(self.left.evaluate(context.clone()), self.right.evaluate(context.clone()))  ,
            OperationType::NotEqual => conditional::not_equal(self.left.evaluate(context.clone()), self.right.evaluate(context.clone()))  ,
            _ => panic!("Cant use this operator")
        }
    }
}

impl ConditionalExp {
    pub fn new(left:Box<dyn Expression> , right:Box<dyn Expression>, op:OperationType) -> ConditionalExp {
        ConditionalExp{left, right , op}
    }
}

