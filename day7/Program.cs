Directory root = new();
List<Directory> allDirs = new();
Stack<Directory> stack = new Stack<Directory>();
stack.Push(root);

foreach (var command in System.IO.File.ReadAllText("../../../input").Split('$', StringSplitOptions.RemoveEmptyEntries))
{
    if (command.StartsWith(" cd /"))
    {
        stack = new Stack<Directory>();
        stack.Push(root);
    }
    else if (command.StartsWith(" cd .."))
    {
        stack.Pop();
    }
    else if (command.StartsWith(" cd "))
    {
        var newTop = new Directory();
        stack.Peek().Contents.Add(newTop);
        stack.Push(newTop);
        allDirs.Add(newTop);
    }
    else if (command.StartsWith(" ls"))
    {
        foreach (var line in command.Split('\n', StringSplitOptions.RemoveEmptyEntries).Skip(1))
        {
            if (line.StartsWith("dir")) continue;
            stack.Peek().Contents.Add(new File()
            {
                Size = long.Parse(line.AsSpan(0, line.IndexOf(' ')))
            });
        }
    }
}
Console.WriteLine($"Part 1: {allDirs.Select(d => d.GetSize()).Where(s => s <= 100000).Sum()}");

long neededSize = 30000000 - (70000000 - root.GetSize());
Console.WriteLine($"Part 2: {allDirs.Select(d => d.GetSize()).Order().First(s => s >= neededSize)}");

interface FileObj
{
    long GetSize();
}

class File : FileObj
{
    public required long Size { get; init; }
    public long GetSize() => Size;
}

class Directory : FileObj
{
    public List<FileObj> Contents { get; } = new();
    public long GetSize() => Contents.Select(c => c.GetSize()).Sum();
}
