// SEM-9 e2e: cross-class clinit ordering.
// Class B's static initializer reads A.VALUE, which requires A's clinit to have run first.
// Expected output: 84

class ClinitA {
    // Non-final so javac does NOT inline the value as a compile-time constant.
    static int VALUE;
    static {
        VALUE = 42;
    }
}

class ClinitB {
    static int RESULT;
    static {
        // This clinit depends on ClinitA being initialized first.
        RESULT = ClinitA.VALUE * 2;
    }
}

public class ClinitOrder {
    public static void main(String[] args) {
        System.out.println(ClinitB.RESULT);
    }
}
