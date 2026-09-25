public class ExceptionChaining {
    static void processNum(String s) {
        try {
            int n = Integer.parseInt(s);
            System.out.println("parsed: " + n);
        } catch (NumberFormatException e) {
            throw new RuntimeException("bad number: " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        processNum("100");
        try {
            processNum("NaN");
        } catch (RuntimeException e) {
            System.out.println(e.getMessage());
        }
        System.out.println("done");
    }
}
