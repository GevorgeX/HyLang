//
// Created by Gev Navoyan on 02.01.25.
//

#include "unaryExpNode.h"

UnaryExpNode::UnaryExpNode(std::unique_ptr<Node> value, UnaryOperation opType):value(std::move(value)),opType(opType)
{
}
