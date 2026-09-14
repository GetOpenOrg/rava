public class TestExceptions {

    static int divide(int a, int b) {
        if (b == 0) throw new ArithmeticException("division by zero");
        return a / b;
    }

    static class AppException extends RuntimeException {
        int code;
        AppException(String msg, int code) {
            super(msg);
            this.code = code;
        }
    }

    public static void main(String[] args) {
        // 正常执行
        try {
            System.out.println(divide(10, 2));
        } catch (ArithmeticException e) {
            System.out.println("error: " + e.getMessage());
        }

        // 捕获异常
        try {
            System.out.println(divide(10, 0));
        } catch (ArithmeticException e) {
            System.out.println("caught: " + e.getMessage());
        }

        // finally 块
        try {
            System.out.println("try");
            throw new RuntimeException("boom");
        } catch (RuntimeException e) {
            System.out.println("catch: " + e.getMessage());
        } finally {
            System.out.println("finally");
        }

        // 自定义异常
        try {
            throw new AppException("app error", 404);
        } catch (AppException e) {
            System.out.println(e.getMessage() + " code=" + e.code);
        }

        // NullPointerException
        try {
            String s = null;
            s.length();
        } catch (NullPointerException e) {
            System.out.println("null pointer");
        }

        // ArrayIndexOutOfBoundsException
        try {
            int[] arr = new int[3];
            int x = arr[5];
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("index out of bounds");
        }

        // try-with 多 catch（父类在后）
        try {
            throw new IllegalArgumentException("bad arg");
        } catch (IllegalArgumentException e) {
            System.out.println("illegal: " + e.getMessage());
        } catch (RuntimeException e) {
            System.out.println("runtime: " + e.getMessage());
        }

        System.out.println("done");
    }
}
