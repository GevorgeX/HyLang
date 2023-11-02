using System.Collections.Generic;

namespace HyLang.Parser.AST;

public  class Node
{
    public List<Node> nodes =  new();
    public void Add(Node node) => nodes.Add(node);
    public override string? ToString()
    {
        return GetType().Name;
    }
    public static void PrintTree(Node tree, string indent = "\n", bool last = true)
    {
        System.Console.Write(indent + "└── " + tree);
        indent += last ? "   " : "|  ";

        for (int i = 0; i < tree.nodes.Count; i++)
        {
            PrintTree(tree.nodes[i], indent, i == tree.nodes.Count - 1);
        }
    }
}