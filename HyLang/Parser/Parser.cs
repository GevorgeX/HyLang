using System.Collections.Generic;
using System.Linq;

namespace HyLang.Parser;

public class Parser
{
    private int _position = 0;
    private List<Token> _tokens;

    public Parser(List<Token> tokens) => _tokens = tokens;

    public void Parse()
    {
        while (_tokens.Count > _position)
        {
            
        }
    }
    public Token GetCurrentToken(int relativePos = 0) => _tokens[_position+relativePos];

    public Token? TokenIs(params TokenType[] types)
    {
        var token = GetCurrentToken();
        if (types.Contains(token.Type))
        {
            _position++;
            return token;
        }
        return null;
    }
}