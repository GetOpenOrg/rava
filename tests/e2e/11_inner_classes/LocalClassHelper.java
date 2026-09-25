public class LocalClassHelper {
    static int processWithLocal(int x) {
        class Multiplier {
            int factor;
            Multiplier(int f) { this.factor = f; }
            int multiply(int n) { return n * factor; }
        }
        Multiplier m = new Multiplier(x);
        return m.multiply(x + 1);
    }

    public static void main(String[] args) {
        System.out.println(processWithLocal(3));  // 3 * 4 = 12
        System.out.println(processWithLocal(5));  // 5 * 6 = 30
    }
}
