#ifndef NODE_H
#define NODE_H

class Visitor;

struct Node
{
    virtual ~Node() = default;
    virtual void Accept(Visitor&) = 0;
};
#endif //NODE_H
