using HyLang.Parser;
using System;

var lexer = new Lexer();

string input = "5+5";

foreach(var el in lexer.GetTokens(input))
    Console.WriteLine(el);

