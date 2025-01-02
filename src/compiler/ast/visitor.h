#ifndef VISITOR_H
#define VISITOR_H
#include "nodes/binaryExpNode.h"
#include "nodes/integerExpNode.h"

class Visitor {
public:
    virtual ~Visitor() = default;
    virtual void visit(IntegerExpNode*);
    virtual void visit(BinaryExpNode*);
};

#endif //VISITOR_H
