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

            #region func# Inner C
            public readonly int field1;
            public readonly Pair<string, double> field2;
            private readonly Tags tag;
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

        #region func# Outer CWLP
        public readonly int field3 = 0;
        public readonly Pair<string, int> field4 = new Pair<string, int>(String.Empty, 0);
        private readonly Tags tag;    //hello

            #region #generated
                //...
                string blag = "";
                Inner(blag);
            #endregion
        #endregion

        ///docu
        public R ExistingFunc<R>(string[] params, int param, Func<T, R> fun)
        {

            return fun();
        }
    }

}