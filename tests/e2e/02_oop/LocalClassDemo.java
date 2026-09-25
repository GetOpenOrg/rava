public class LocalClassDemo {
    public static void main(String[] args) {
        class Multiplier {
            int factor;
            Multiplier(int f) { this.factor = f; }
            int multiply(int x) { return factor * x; }
        }
        Multiplier m = new Multiplier(3);
        System.out.println(m.multiply(7)); // 21
        System.out.println(m.multiply(5)); // 15
    }
}
