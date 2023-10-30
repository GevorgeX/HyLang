using HyLang.Parser;
using System;

var lexer = new Lexer();

string input = "5+5;";

var parser = new Parser(lexer.GetTokens(input));



