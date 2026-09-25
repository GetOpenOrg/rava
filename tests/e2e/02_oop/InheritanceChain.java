public class InheritanceChain {
    // Demonstrates a chain of static method calls across multiple levels.
    // Each level adds to the accumulated result, testing that cross-method
    // call dispatch and return value propagation are correct.

    static int base(int n) {
        return n;
    }

    static int level1(int n) {
        return base(n) + base(n - 1);
    }

    static int level2(int n) {
        return level1(n) + level1(n - 1);
    }

    static int level3(int n) {
        return level2(n) + level2(n - 1);
    }

    public static void main(String[] args) {
        System.out.println(base(5));    // 5
        System.out.println(level1(5));  // 5+4 = 9
        System.out.println(level2(5));  // 9+7 = 16
        System.out.println(level3(5));  // 16+12 = 28
    }
}
