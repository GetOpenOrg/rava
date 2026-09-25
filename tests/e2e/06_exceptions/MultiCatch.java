public class MultiCatch {
    static void risky(int i) throws Exception {
        if (i == 1) throw new IllegalArgumentException("bad arg");
        if (i == 2) throw new ArithmeticException("math error");
        if (i == 3) throw new Exception("checked error");
    }

    public static void main(String[] args) {
        for (int i = 0; i <= 3; i++) {
            try {
                risky(i);
                System.out.println("ok");
            } catch (IllegalArgumentException | ArithmeticException e) {
                System.out.println("multi: " + e.getMessage());
            } catch (Exception e) {
                System.out.println("single: " + e.getMessage());
            }
        }
    }
}
