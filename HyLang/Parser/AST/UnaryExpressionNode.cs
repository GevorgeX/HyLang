using System;
using System.Collections.Generic;

namespace HyLang.Parser.AST;

public class UnaryExpressionNode : Node
{
    public string operation;
    public Node right;

    public UnaryExpressionNode( string operation, Node right)
    {
        // this.left = left;
        this.operation = operation;
        this.right = right;
        // Add(right);
    }
    public override List<Node> GetChildNodes()
    {
        return new List<Node>{right};
    }
    public override string ToString()
    {
        return base.ToString() + $" {operation}";
    }
}
