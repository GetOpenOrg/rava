public class TestMultiArray {
    public static void main(String[] args) {
        // 2D array
        int[][] matrix = {
            {1, 2, 3},
            {4, 5, 6},
            {7, 8, 9}
        };

        // Print matrix
        for (int[] row : matrix) {
            for (int v : row) {
                System.out.print(v + " ");
            }
            System.out.println();
        }

        // Transpose
        int[][] transposed = new int[3][3];
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                transposed[j][i] = matrix[i][j];
            }
        }

        System.out.println("---");
        for (int[] row : transposed) {
            for (int v : row) {
                System.out.print(v + " ");
            }
            System.out.println();
        }

        // Jagged array
        int[][] jagged = new int[3][];
        for (int i = 0; i < 3; i++) {
            jagged[i] = new int[i + 1];
            for (int j = 0; j <= i; j++) {
                jagged[i][j] = i + j;
            }
        }
        for (int[] row : jagged) {
            System.out.println(row.length);
        }
    }
}
