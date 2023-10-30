namespace HyLang.Parser.AST;

public class NumberNode: Node
{
    public double number;

    public NumberNode(string number)
    {
        this.number = double.Parse(number);
    }
    
}