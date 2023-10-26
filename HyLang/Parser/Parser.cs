using System.Collections.Generic;

namespace HyLang.Parser;

public class Parser
{
    private int _position = 0;
    private List<Token> _tokens;

    public Parser(List<Token> tokens) => _tokens = tokens;

    public Token GetCurrentToken(int relativePos = 0) => _tokens[_position+relativePos];
    public void NextToken() => _position++;
}