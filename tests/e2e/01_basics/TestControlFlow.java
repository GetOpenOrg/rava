public class TestControlFlow {
    public static void main(String[] args) {
        // if/else
        int x = 10;
        if (x > 5) {
            System.out.println("greater");
        } else {
            System.out.println("smaller");
        }

        // while loop
        int i = 0;
        while (i < 3) {
            System.out.println(i);
            i++;
        }

        // for loop with break
        for (int j = 0; j < 10; j++) {
            if (j == 5) break;
            System.out.println(j);
        }

        // for loop with continue
        for (int k = 0; k < 5; k++) {
            if (k == 2) continue;
            System.out.println(k);
        }

        // switch
        int day = 3;
        switch (day) {
            case 1: System.out.println("Monday");    break;
            case 2: System.out.println("Tuesday");   break;
            case 3: System.out.println("Wednesday"); break;
            default: System.out.println("Other");    break;
        }

        // ternary
        String result = x > 5 ? "yes" : "no";
        System.out.println(result);

        // do-while
        int n = 0;
        do {
            System.out.println("do " + n);
            n++;
        } while (n < 2);

        // nested loops
        for (int a = 0; a < 2; a++) {
            for (int b = 0; b < 2; b++) {
                System.out.println(a + "," + b);
            }
        }
    }
}
