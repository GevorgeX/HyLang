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

    public List<Node> ParseCode()
    {
        var root = new List<Node>();
        while (_position < _tokens.Count)
        {
            var expression = Expression();
            if (!MatchToken(TokenType.End))
            {
                throw new Exception();
            }
            root.Add(expression);
        }

        return root;
    }

    private Node Expression()
    {
        return Additive();
    }

    private Node Additive(){
        var result = Multiplicative();
                while (true)
        {
            if( MatchToken(TokenType.Plus)){
                result = new BinaryExpressionNode(result ,"+" , Multiplicative() );
            }
            else if( MatchToken(TokenType.Minus)){
                result = new BinaryExpressionNode(result ,"-" , Multiplicative() );
            }
            else{
                break;
            }
        }
        return result;
    }
    private Node Multiplicative(){
        var result = Unary();
        while (true)
        {
            if( MatchToken(TokenType.Multiply)){
                result = new BinaryExpressionNode(result ,"*" , Unary() );
            }
            else if( MatchToken(TokenType.Divide)){
                result = new BinaryExpressionNode(result ,"/" , Unary() );
            }
            else{
                break;
            }
        }
        return result;
    }
    private Node Unary(){

        var result = Primary();
        return result;
    }
    private Node Primary(){
        var token = GetCurrentToken();
        if(MatchToken(TokenType.Number)){
            return new NumberExpressionNode(token.Text);
        }
        else if(MatchToken(TokenType.LeftParenthesis)){
            var result = Expression();
            RequireToken(TokenType.RightParenthesis);
            return result;
        }
        throw new Exception("Symbol error");
    }


    private Token GetCurrentToken(int relativePos = 0)
    {
        var token = _tokens[_position+relativePos];
        return token;
    }
    private void RequireToken(params TokenType[] types){
        if(!MatchToken(types))
            throw new Exception($"Require {types[0]}");
    }
    private bool MatchToken(params TokenType[] types)
    
    {
        var token = GetCurrentToken();
        if (types.Contains(token.Type))
        {
            _position++;
            return true;
        }

        return false;
    }
}