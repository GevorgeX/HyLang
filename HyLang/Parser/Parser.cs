using System;
using System.Collections.Generic;
using System.Linq;
using HyLang.Parser.AST;

namespace HyLang.Parser;

public class Parser
{
    private int _position = 0;
    private List<Token> _tokens;

    public Parser(List<Token> tokens) => _tokens = tokens;

    public Node ParseCode()
    {
        var root = new Node();
        while (_position < _tokens.Count)
        {
            var expression = ParseExpression();
            if (MatchToken(TokenType.End) is null)
            {
                throw new Exception();
            }
            root.Add(expression);
        }

        return root;
    }

    private Node ParseExpression()
    {
    
    }

    private Token GetCurrentToken(int relativePos = 0)
    {
        var token = _tokens[_position+relativePos];
        return token;
    }
    private Token? MatchToken(params TokenType[] types)
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