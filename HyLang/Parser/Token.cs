namespace HyLang.Parser;

public struct Token
{
    public readonly string Text;
    public readonly TokenType Type;

    public Token(string text , TokenType type)
    {
        Text = text;
        Type = type;
    }

    public override string ToString()
    {
        return $"Type = {Type} , Value = {Text}";
    }
}