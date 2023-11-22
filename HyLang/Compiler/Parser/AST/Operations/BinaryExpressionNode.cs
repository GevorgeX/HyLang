using System.Collections.Generic;

namespace HyLang.Compiler.Parser.AST.Operations;

public sealed class BinaryExpressionNode:ExpressionNode
{
    public ExpressionNode left { get; }
    public ExpressionNode right { get; }
    public string operation { get; }

    public BinaryExpressionNode(ExpressionNode left, string operation, ExpressionNode right)
    {
        this.left = left;
        this.operation = operation;
        this.right = right;
        // Add(left);
        // Add(right);
    }
    public override double Eval()
    {
        return operation switch 
        {
            "+" => left.Eval() + right.Eval(),
            "-" => left.Eval() - right.Eval(),
            "*" => left.Eval() * right.Eval(),
            "/" => left.Eval() / right.Eval(),
            _ => throw new System.Exception(),

        };
    }
    public override List<Node> GetChildNodes()
    {
        return new List<Node>{left , right};
    }
    public override string ToString()
    {
        return base.ToString() + $" {operation}";
    }
}