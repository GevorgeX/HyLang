using System.Collections.Generic;

namespace HyLang.Parser.AST;

public class Node
{
    public virtual List<Node> GetChildNodes() =>  new List<Node>();
    public override string? ToString()
    {
        return GetType().Name;
    }
    public static void PrintTree(Node tree, string indent = "\n", bool last = true)
    {
        System.Console.Write(indent + "└──── " + tree);
        indent += last ? "   " : "|  ";

        for (int i = 0; i < tree.GetChildNodes().Count; i++)
        {
            PrintTree(tree.GetChildNodes()[i], indent, i == tree.GetChildNodes().Count - 1);
        }
    }
}