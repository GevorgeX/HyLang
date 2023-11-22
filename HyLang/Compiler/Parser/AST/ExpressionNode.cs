namespace HyLang.Compiler.Parser.AST;

public abstract class ExpressionNode : Node
{
    public abstract double Eval();
}