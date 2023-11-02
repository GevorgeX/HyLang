using HyLang.Parser;
using HyLang.Parser.AST;
using System;

var lexer = new Lexer();

string input = "5+5;";

var parser = new Parser(lexer.GetTokens(input));


Node.PrintTree(parser.ParseCode()); 
