using System.Collections.Generic;
using System.Text.RegularExpressions;
using System.Linq;

namespace HyLang.Parser;

public class Lexer
{
    public static readonly string[] Delimiters = {" ","+" , "-" , "*" , "/",";" ,"(" ,")"};
    
    private static readonly string Pattern ='(' +string.Join("|", Delimiters.Select(Regex.Escape)) +')';
    public string[] Parse(string text)
    {
        var res = Regex.Split(text, Pattern);
        return res.Where(el => !string.IsNullOrEmpty(el) && !string.IsNullOrWhiteSpace(el) ).ToArray();
    }

    public List<Token> GetTokens(string text)
    {
        var parsedText = Parse(text);

        List<Token> res = new();
        for (int i = 0; i < parsedText.Length; i++)
        {
            string txt = parsedText[i];
            res.Add(new Token(txt, KeyWords.GetTokenType(txt)));
        }

        return res;
    }
    
}