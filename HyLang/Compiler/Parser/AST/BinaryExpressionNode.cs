using System.Collections.Generic;

namespace HyLang.Compiler.Parser.AST;

public class BinaryExpressionNode:Node
{
    public Node left;
    public Node right;
    public string operation;

    public BinaryExpressionNode(Node left, string operation, Node right)
    {
        this.left = left;
        this.operation = operation;
        this.right = right;
        // Add(left);
        // Add(right);
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