using System.Collections.Generic;

namespace HyLang.Compiler.Parser.AST.Operations;

public sealed class NumberExpressionNode: ExpressionNode
{
    public string number { get; }

    public NumberExpressionNode(string number)
    {
        this.number = number;
    }

    public override double Eval()
    {
        return double.Parse(number);
    }

    public override List<Node> GetChildNodes() => new List<Node>();

    public override string ToString()
    {
        return base.ToString() + $" {number}";
    }
}