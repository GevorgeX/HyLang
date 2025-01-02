#include "binaryExpNode.h"
#include "../visitor.h"

BinaryExpNode::BinaryExpNode(std::unique_ptr<Node> left, std::unique_ptr<Node> right, BinaryOperation opType):left(std::move(left)),right(std::move(right)),opType(opType)
{
}

void BinaryExpNode::Accept(Visitor& visitor)
{
    left->Accept(visitor);
    right->Accept(visitor);
    visitor.visit(this);
}
