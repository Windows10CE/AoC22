using System.Diagnostics;

var graph = File.ReadAllLines("../../../input")
    .Select(line => line.Select(c => new Node() { Value = c - '0' }).ToArray())
    .ToArray();

for (int i = 0; i < graph.Length; i++)
{
    var row = graph[i];
    for (int j = 0; j < row.Length; j++)
    {
        var node = row[j];
        if (i > 0)
        {
            node.Up = graph[i - 1][j];
        }
        if (i < graph.Length - 1)
        {
            node.Down = graph[i + 1][j];
        }
        if (j > 0)
        {
            node.Left = graph[i][j - 1];
        }
        if (j < row.Length - 1)
        {
            node.Right = graph[i][j + 1];
        }
    }
}

HashSet<Node> visibleNodes = new();

for (int iRow = 0; iRow < graph.Length; iRow++)
{
    Node biggestLeft = graph[iRow][0];
    Node biggestRight = graph[iRow][^1];
    visibleNodes.Add(biggestLeft);
    visibleNodes.Add(biggestRight);

    Node current = biggestLeft;
    while (current.Right != null)
    {
        if (current.Right.Value > biggestLeft.Value)
        {
            biggestLeft = current.Right;
            visibleNodes.Add(biggestLeft);
        }
        current = current.Right;
    }
    current = biggestRight;
    while (current.Left != null)
    {
        if (current.Left.Value > biggestRight.Value)
        {
            biggestRight = current.Left;
            visibleNodes.Add(biggestRight);
        }
        current = current.Left;
    }
}
for (int iColumn = 0; iColumn < graph[0].Length; iColumn++)
{
    Node biggestUp = graph[0][iColumn];
    Node biggestDown = graph[^1][iColumn];
    visibleNodes.Add(biggestUp);
    visibleNodes.Add(biggestDown);

    Node current = biggestUp;
    while (current.Down != null)
    {
        if (current.Down.Value > biggestUp.Value)
        {
            biggestUp = current.Down;
            visibleNodes.Add(biggestUp);
        }
        current = current.Down;
    }
    current = biggestDown;
    while (current.Up != null)
    {
        if (current.Up.Value > biggestDown.Value)
        {
            biggestDown = current.Up;
            visibleNodes.Add(biggestDown);
        }
        current = current.Up;
    }
}

Console.WriteLine($"Part 1: {visibleNodes.Count}");

List<int> scores = new(graph.Length * graph[0].Length);
foreach (var row in graph)
{
    foreach (var node in row)
    {
        int up = 0, down = 0, left = 0, right = 0;
        Node? current = node.Left;
        while (current != null)
        {
            left++;
            if (current.Value >= node.Value)
            {
                break;
            }
            current = current.Left;
        }
        current = node.Right;
        while (current != null)
        {
            right++;
            if (current.Value >= node.Value)
            {
                break;
            }
            current = current.Right;
        }
        current = node.Up;
        while (current != null)
        {
            up++;
            if (current.Value >= node.Value)
            {
                break;
            }
            current = current.Up;
        }
        current = node.Down;
        while (current != null)
        {
            down++;
            if (current.Value >= node.Value)
            {
                break;
            }
            current = current.Down;
        }
        
        scores.Add(left * right * up * down);
    }
}
Console.WriteLine($"Part 2: {scores.OrderDescending().First()}");

class Node
{
    public required int Value { get; init; }
    public Node? Left { get; set; }
    public Node? Right { get; set; }
    public Node? Up { get; set; }
    public Node? Down { get; set; }
}
