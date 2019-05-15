using System.Generic;

namespace ns.dummy
{
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

            #region #func Inner
            public readonly int field;
            public readonly Pair<string, double> field;
            private readonly Tags tag;

            #region #generated
            //...
            #endregion
            #endregion
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

        #region func# Outer
        public readonly int field = 0;
        public readonly Pair<string, int> field = new Pair<string, int>(String.Empty, 0);
        private readonly Tags tag;    //hello
        #endregion

        ///docu
        public R ExistingFunc<R>(string[] params, int param, Func<T, R> fun)
        {

            return fun();
        }
    }

}