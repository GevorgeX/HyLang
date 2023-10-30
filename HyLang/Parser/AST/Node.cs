using System.Collections.Generic;

namespace HyLang.Parser.AST;

public  class Node
{
    public List<Node> nodes =  new();
    public void Add(Node node) => nodes.Add(node);
}