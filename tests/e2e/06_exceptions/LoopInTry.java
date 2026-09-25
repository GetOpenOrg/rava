public class LoopInTry {
    public static void main(String[] args) {
        // 1. Loop inside try: exception thrown mid-loop, exits loop to catch.
        try {
            for (int i = 0; i < 5; i++) {
                if (i == 3) {
                    throw new RuntimeException("loop error at " + i);
                }
                System.out.println("loop: " + i);
            }
            System.out.println("loop completed (should not reach)");
        } catch (RuntimeException e) {
            System.out.println("caught: " + e.getMessage());
        }

        // 2. Try inside loop: each iteration catches its own exception independently.
        for (int i = 0; i < 3; i++) {
            try {
                if (i == 1) {
                    throw new RuntimeException("error at " + i);
                }
                System.out.println("ok: " + i);
            } catch (RuntimeException e) {
                System.out.println("caught in loop: " + e.getMessage());
            }
        }

        // 3. While loop inside try; exception on last iteration.
        int n = 0;
        try {
            while (n < 4) {
                if (n == 3) {
                    throw new RuntimeException("while error");
                }
                System.out.println("while: " + n);
                n++;
            }
        } catch (RuntimeException e) {
            System.out.println("while caught: " + e.getMessage());
        }

        System.out.println("done");
    }
}
