class Node {
    var head: int
    var l: seq<int>
}

class Program {
    var l: seq<Node>
    method foo(x: int) returns (y: int)
      ensures y == x + 1
    {
        return x + 1;
    }
}