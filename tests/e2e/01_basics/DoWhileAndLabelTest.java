public class DoWhileAndLabelTest {
    public static void main(String[] args) {
        // Test 1: Basic do-while
        int i = 0;
        int sum = 0;
        do {
            sum += i;
            i++;
        } while (i < 5);
        System.out.println("do-while sum: " + sum); // 10

        // Test 2: do-while with single iteration
        int count = 0;
        do {
            count++;
        } while (false);
        System.out.println("single iteration: " + count); // 1

        // Test 3: do-while with break
        int val = 0;
        do {
            val++;
            if (val == 3) break;
        } while (val < 10);
        System.out.println("break at: " + val); // 3

        // Test 4: do-while with continue
        StringBuilder sb = new StringBuilder();
        int j = 0;
        do {
            j++;
            if (j % 2 == 0) continue;
            sb.append(j);
        } while (j < 6);
        System.out.println("odd: " + sb.toString()); // 135

        // Test 5: Labeled break from nested loop
        int found = -1;
        int[][] matrix = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
        SEARCH:
        for (int r = 0; r < matrix.length; r++) {
            for (int c = 0; c < matrix[r].length; c++) {
                if (matrix[r][c] == 5) {
                    found = r * 10 + c;
                    break SEARCH;
                }
            }
        }
        System.out.println("found 5 at: " + found); // 11

        // Test 6: Labeled continue
        StringBuilder result = new StringBuilder();
        OUTER:
        for (int x = 0; x < 3; x++) {
            for (int y = 0; y < 3; y++) {
                if (y == 1) continue OUTER;
                result.append(x).append(y).append(" ");
            }
        }
        System.out.println("labeled continue: " + result.toString().trim()); // 00 10 20

        // Test 7: Nested do-while
        int a = 1;
        int b;
        StringBuilder nested = new StringBuilder();
        do {
            b = 1;
            do {
                nested.append(a * b).append(" ");
                b++;
            } while (b <= 3);
            a++;
        } while (a <= 3);
        System.out.println("nested: " + nested.toString().trim());
        // 1 2 3 2 4 6 3 6 9

        // Test 8: for-loop with labeled break (outer label)
        int total = 0;
        DONE:
        for (int m = 1; m <= 10; m++) {
            for (int n = 1; n <= 10; n++) {
                total += m * n;
                if (total > 50) break DONE;
            }
        }
        System.out.println("total: " + total); // 55
    }
}
