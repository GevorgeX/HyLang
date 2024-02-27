use std::rc::Rc;

use crate::interpreter::library::{object_utils::arithmetical, Context, ReferenceToObject};

use super::{Expression, OperationType};

pub struct BinaryExp{
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    op: OperationType
}

impl super::Expression for BinaryExp {
    fn evaluate(&self,context:Rc<Context>) -> ReferenceToObject {
        match self.op {
            OperationType::Plus => arithmetical::add(self.left.evaluate(context.clone()), self.right.evaluate(context.clone())) ,
            OperationType::Minus => arithmetical::sub(self.left.evaluate(context.clone()), self.right.evaluate(context.clone())) ,
            OperationType::Multi => arithmetical::mult(self.left.evaluate(context.clone()), self.right.evaluate(context.clone())) ,
            OperationType::Divide => arithmetical::div(self.left.evaluate(context.clone()), self.right.evaluate(context.clone())) ,
            _ => panic!("Cant use this operator")
        }
    }
}

impl BinaryExp {
    pub fn new(left:Box<dyn Expression> , right:Box<dyn Expression>, op:OperationType) -> BinaryExp {
        BinaryExp{left, right , op}
    }
}

