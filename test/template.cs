using System.Generic;

namespace ns.dummy {

public abstract class Outer
{
    class Inner 
    {
        private enum Tags
        {
            First,
            Second,
            Third,
            Fourth,
            Last
        }

        public readonly int field;
        public readonly ns.pair<ns.string, double> field;
        private readonly Tags tag;
    }

    internal enum Tags
    {
        First,
        Second,
        /*
        Third,
        Fourth, */
        Last
    }

    public readonly int field;
    public readonly ns.pair<namespace.string> field;
    private readonly Tags tag;    //hello

    ///docu
    public R ExistingFunc<R>(string[] params, int param, Func<T,R> fun) 
    {
        
        return fun();
    }
}

}