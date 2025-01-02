#ifndef BINARYEXPNODE_H
#define BINARYEXPNODE_H
#include <memory>

#include "../node.h"
#include "../operations.h"

struct BinaryExpNode : Node {
    std::unique_ptr<Node> left;
    std::unique_ptr<Node> right;
    BinaryOperation opType;
    BinaryExpNode(std::unique_ptr<Node>, std::unique_ptr<Node>, BinaryOperation);
    void Accept(Visitor&) override;
};

#endif //BINARYEXPNODE_H
