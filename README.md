# Call this tool 'func sharp' | 'fun C#' | 'fancy sharp' as you like!
Generates boilerplate code into C# .cs files for functional programming friendly readonly structs/classes

## Usage from command line:
```
funcsharp.exe <input file name> <output file name>
```
Where output file name may be the same as input file name. 

## Usage in the code
```
#region func# <you class name> <options>
  <your fields / properties>
#endregion
```

## Possible options:
    C - public constructor generation for every fields
    c - private constructor generation for every fields
    W - With(...) generation for every fields at once
    w - WithXXX(.) generation for every fields separately
    P - property getter generation for every fields"#)

## Example Input:
```
    public struct Position2d
    {
#region func# Position2d CWwP
        public readonly double x;
        public readonly double y;
#endregion
        
        public static readonly Position2d Origo = new Position2d(0, 0);
    }
```
## Example Output:
```
    public struct Position2d
    {
#region func# Position2d CWwP
        public readonly double x;
        public readonly double y;
#region #generated 56338F7B 0.1.0
        public double X { get { return this.x; } }
        public double Y { get { return this.y; } }

        public Position2d(double x, double y)
        {
            this.x = x;
            this.y = y;
        }

        public Position2d With(double x = null, double y = null)
        {
            return new Position2d(
                x: x ?? this.x,
                y: y ?? this.y);
        }
        public Position2d WithX(double x)
        {
            return new Position2d(
                x: x,
                y: this.y);
        }
        public Position2d WithY(double y)
        {
            return new Position2d(
                x: this.x,
                y: y);
        }
#endregion
#endregion
        
        public static readonly Position2d Origo = new Position2d(0, 0);
    }
```
