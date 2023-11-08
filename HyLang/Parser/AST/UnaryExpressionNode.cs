using System;

namespace HyLang.Parser.AST;

public class UnaryExpressionNode : Node
{
    public string operation;

    public UnaryExpressionNode( string operation, Node right)
    {
        // this.left = left;
        this.operation = operation;
        // this.right = right;
        Add(right);
    }
        public override string ToString()
    {
        return base.ToString() + $" Operation -> {operation}";
    }
}
