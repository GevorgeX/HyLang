#ifndef INTEGEREXPNODE_H
#define INTEGEREXPNODE_H
#include "../node.h"

struct IntegerExpNode : Node{
    int value;
    void Accept(Visitor&) override;
    IntegerExpNode(int);
};



#endif //INTEGEREXPNODE_H
