public class InnerClassTest {

    private String name;
    private int value;

    class Inner {
        private int innerValue;

        Inner(int val) {
            this.innerValue = val;
        }

        void display() {
            // Access outer class fields
            System.out.println("Outer name: " + name);
            System.out.println("Outer value: " + value);
            System.out.println("Inner value: " + innerValue);
        }

        int getSum() {
            return value + innerValue;
        }
    }

    InnerClassTest(String name, int value) {
        this.name = name;
        this.value = value;
    }

    public static void main(String[] args) {
        InnerClassTest outer = new InnerClassTest("Test", 10);
        InnerClassTest.Inner inner = outer.new Inner(20);
        inner.display();
        System.out.println("Sum: " + inner.getSum());

        // Create another inner with different outer
        InnerClassTest outer2 = new InnerClassTest("Other", 100);
        InnerClassTest.Inner inner2 = outer2.new Inner(50);
        inner2.display();
        System.out.println("Sum: " + inner2.getSum());
    }
}
