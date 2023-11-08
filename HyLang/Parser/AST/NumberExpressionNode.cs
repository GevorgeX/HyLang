namespace HyLang.Parser.AST;

public class NumberExpressionNode: Node
{
    public double number;

    public NumberExpressionNode(string number)
    {
        this.number = double.Parse(number);
    }
    public override string ToString()
    {
        return base.ToString() + $" {number}";
    }
}