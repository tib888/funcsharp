# FunC# | FuncySharp | func C3
Generates boilerplate code into C# .cs files for functional programming friendly readonly structs/classes

# Usage:
#region func# <yor class name> <options>
  <your fields / properties>
#endregion
    
# Possible options:
    C - *public* constructor generation for every fields
    C - *private* constructor generation for every fields
    W - With(...) generation for every fields
    L - Lens generation for every fields
    P - property getter generation for every fields

# Example Input:

    public struct Position2d
    {
#region func# Position2d CWLP
        public readonly double x;
        public readonly double y;
#endregion
        
        public static readonly Position2d Origo = new Position2d(0, 0);

        public Position To3D(TrMatrix tr)
        {
            return tr.Transform(new Position(x, y, 0));
        }
    }

# Example Output:

    public struct Position2d
    {
#region func# Position2d CWLP
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

        public Position To3D(TrMatrix tr)
        {
            return tr.Transform(new Position(x, y, 0));
        }
    }
