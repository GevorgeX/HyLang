using System.Collections.Generic;

namespace HyLang.Compiler.Parser.AST.Operations;

public sealed class UnaryExpressionNode : ExpressionNode
{
    public string operation { get; }
    public ExpressionNode right { get; }

    public UnaryExpressionNode( string operation, ExpressionNode right)
    {
        // this.left = left;
        this.operation = operation;
        this.right = right;
        // Add(right);
    }

    public override double Eval()
    {
        return operation switch
        {
            "-" => -right.Eval(),
            _ => right.Eval()  
        };
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
