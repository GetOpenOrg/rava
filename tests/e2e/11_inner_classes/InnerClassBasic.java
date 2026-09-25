/**
 * Tests non-static inner classes: the inner class holds an implicit outer reference
 * (this$0 field) which the transpiler renames to outer_ref.
 * Focuses on inner-class own-field access without outer member access.
 */
public class InnerClassBasic {
    private int outerValue;

    public InnerClassBasic(int v) {
        this.outerValue = v;
    }

    class Counter {
        private int count;

        Counter(int start) {
            this.count = start;
        }

        void increment() {
            this.count++;
        }

        void add(int n) {
            this.count += n;
        }

        int getCount() {
            return this.count;
        }
    }

    public static void main(String[] args) {
        InnerClassBasic outer = new InnerClassBasic(100);
        InnerClassBasic.Counter c = outer.new Counter(5);
        c.increment();
        c.increment();
        System.out.println(c.getCount());  // 7
        c.add(3);
        System.out.println(c.getCount());  // 10
    }
}
