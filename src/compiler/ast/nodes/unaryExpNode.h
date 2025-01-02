#ifndef UNARYEXPNODE_H
#define UNARYEXPNODE_H
#include <memory>
#include "../node.h"
#include "../operations.h"

struct UnaryExpNode {
    std::unique_ptr<Node> value;
    UnaryOperation opType;
    UnaryExpNode(std::unique_ptr<Node>, UnaryOperation);
};

#endif //UNARYEXPNODE_H
