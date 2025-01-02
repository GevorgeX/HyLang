#include "integerExpNode.h"
#include "../visitor.h"
void IntegerExpNode::Accept(Visitor& visitor)
{
    visitor.visit(this);
}

IntegerExpNode::IntegerExpNode(int value):value(value)
{
}
