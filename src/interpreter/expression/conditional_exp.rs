use crate::interpreter::library::object_utils::{conditional, logical};
use crate::interpreter::library::{Context, ReferenceToObject};

use super::{Expression, OperationType};

pub struct ConditionalExp{
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    op: OperationType
}

impl super::Expression for ConditionalExp {
    fn evaluate(&self,context:&Context) -> ReferenceToObject {
        match self.op {
            OperationType::And => logical::and(self.left.evaluate(context), self.right.evaluate(context))  ,
            OperationType::Or =>logical::or(self.left.evaluate(context), self.right.evaluate(context))  ,
            OperationType::More =>conditional::more(self.left.evaluate(context), self.right.evaluate(context))  ,
            OperationType::Less =>conditional::less(self.left.evaluate(context), self.right.evaluate(context))  ,
            OperationType::MoreOrEq =>conditional::more_or_eq(self.left.evaluate(context), self.right.evaluate(context))  ,
            OperationType::LessOrEq =>conditional::less_or_eq(self.left.evaluate(context), self.right.evaluate(context))  ,
            OperationType::DoubleEqual => conditional::equal(self.left.evaluate(context), self.right.evaluate(context))  ,
            OperationType::NotEqual => conditional::not_equal(self.left.evaluate(context), self.right.evaluate(context))  ,
            _ => panic!("Cant use this operator")
        }
    }
}

impl ConditionalExp {
    pub fn new(left:Box<dyn Expression> , right:Box<dyn Expression>, op:OperationType) -> ConditionalExp {
        ConditionalExp{left, right , op}
    }
}

