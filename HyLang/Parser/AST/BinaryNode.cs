namespace HyLang.Parser.AST;

public class BinaryNode:Node
{
    public NumberNode left;
    public NumberNode right;
    public string operation;

    public BinaryNode(NumberNode left, string operation, NumberNode right)
    {
        this.left = left;
        this.operation = operation;
        this.right = right;
    }
}