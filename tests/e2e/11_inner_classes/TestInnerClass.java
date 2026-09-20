public class TestInnerClass {

    private String outerField = "outer-field";
    private int counter = 0;
    private static String staticOuter = "static-outer";

    // 非静态内部类：可访问外部私有成员
    class Inner {
        private String innerField = "inner-field";

        String readOuter() {
            return outerField + "/" + staticOuter;
        }

        void bumpOuter() {
            counter++;
            TestInnerClass.this.counter++;
        }

        int outerCount() {
            return TestInnerClass.this.counter;
        }

        String self() {
            return innerField;
        }
    }

    // 内部类的内部类（两级嵌套）
    class Middle {
        class Innermost {
            String chain() {
                return outerField + ">" + "middle>" + "innermost";
            }
        }
    }

    Inner makeInner() {
        return new Inner();
    }

    Inner makeAnonymousSub() {
        return new Inner() {
            String extra() {
                return "anon-extends-inner:" + readOuter();
            }
        };
    }

    public static void main(String[] args) {
        TestInnerClass outer = new TestInnerClass();

        Inner i1 = outer.new Inner();
        System.out.println(i1.readOuter());
        System.out.println(i1.self());
        i1.bumpOuter();
        System.out.println("counter=" + outer.counter);
        System.out.println("outerCount=" + i1.outerCount());

        Inner i2 = outer.makeInner();
        i2.bumpOuter();
        System.out.println("after second inner counter=" + outer.counter);

        // 两个内部类实例共享同一个外部实例
        System.out.println("i1 sees=" + i1.outerCount() + " i2 sees=" + i2.outerCount());

        Middle.Innermost deep = outer.new Middle().new Innermost();
        System.out.println(deep.chain());

        // extends 内部类的匿名子类
        System.out.println(outer.makeAnonymousSub().readOuter());

        // 静态嵌套类作为对照：不持有外部实例
        StaticHelper.print("from static nested");
        System.out.println(StaticHelper.name);

        System.out.println("done");
    }

    static class StaticHelper {
        static String name = "StaticHelper";

        static void print(String s) {
            System.out.println("StaticHelper: " + s);
        }
    }
}
