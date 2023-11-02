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
        var token = MatchToken(TokenType.Number , TokenType.Plus);

        switch (token.Value.Type)
        {
            case TokenType.Number:
                return new NumberNode(token.Value.Text);

            case TokenType.Plus:
            {
                var nextToken = MatchToken(TokenType.Number);
                return new BinaryNode(token , )
            }

            default:
                throw new Exception();
        }
    }


    private Token GetCurrentToken(int relativePos = 0)
    {
        var token = _tokens[_position+relativePos];
        return token;
    }
    private TokenType GetCurrentTokenType()
    {
        return _tokens[_position].Type;
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