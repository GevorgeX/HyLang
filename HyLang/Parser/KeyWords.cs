namespace HyLang.Parser;

public static class KeyWords
{
    public static TokenType GetTokenType(string str)
    {
        return str switch
        {
            var a when int.TryParse(a ,out _) => TokenType.Number,
            "+" => TokenType.Plus,
            "-" => TokenType.Minus,
            "*" => TokenType.Multiply,
            "/" => TokenType.Divide,
            ";" => TokenType.End,
            "(" => TokenType.LeftParenthesis,
            ")" => TokenType.RightParenthesis,
            _ => TokenType.Undefined
        };
    }
}