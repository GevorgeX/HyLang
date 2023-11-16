using HyLang.Compiler.Lexer;
using HyLang.Compiler.Parser;

var lexer = new Lexer();

string input = "(5+5)*6 - 4;";

var parser = new Parser(lexer.GetTokens(input));


foreach (var item in parser.ParseCode())
{
    HyLang.Compiler.Parser.AST.Node.PrintTree(item); 
}
