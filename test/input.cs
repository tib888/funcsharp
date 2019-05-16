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
#region #generated 272A7EBB 0.1.0

        public Inner(int field1, Pair<string, double> field2, Tags tag)
        {
            this.field1 = field1;
            this.field2 = field2;
            this.tag = tag;
        }
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

        #region func# Outer CWLP
        public readonly int field3 = 0;
        public readonly Pair<string, int> field4 = new Pair<string, int>(String.Empty, 0);
        private readonly Tags tag;    //hello

            #region #generated C2903276 0.1.0
        public int Field3 { get { return this.field3; } }
        public Pair<string, int> Field4 { get { return this.field4; } }
        public Tags Tag { get { return this.tag; } }

        public Outer(int field3 = 0, Pair<string, int> field4 = new Pair<string, int>(String.Empty, 0), Tags tag)
        {
            this.field3 = field3;
            this.field4 = field4;
            this.tag = tag;
        }

        public Outer With(int field3 = null, Pair<string, int> field4 = null, Tags tag = null)
        {
            return new Outer(
                field3: field3 ?? this.field3,
                field4: field4 ?? this.field4,
                tag: tag ?? this.tag);
        }
        public Outer WithField3(int field3)
        {
            return new Outer(
                field3: field3,
                field4: this.field4,
                tag: this.tag);
        }
        public Outer WithField4(Pair<string, int> field4)
        {
            return new Outer(
                field3: this.field3,
                field4: field4,
                tag: this.tag);
        }
        public Outer WithTag(Tags tag)
        {
            return new Outer(
                field3: this.field3,
                field4: this.field4,
                tag: tag);
        }
#endregion
#endregion

        ///docu
        public R ExistingFunc<R>(string[] params, int param, Func<T, R> fun)
        {

            return fun();
        }
    }

}