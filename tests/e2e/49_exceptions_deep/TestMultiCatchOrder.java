import java.io.IOException;

public class TestMultiCatchOrder {
    static void f(boolean io) throws IOException, RuntimeException {
        if (io) throw new IOException("io");
        throw new RuntimeException("rt");
    }

    public static void main(String[] args) {
        try {
            f(true);
        } catch (IOException e) {
            System.out.println("io=" + e.getMessage());
        } catch (Exception e) {
            System.out.println("other=" + e.getMessage());
        }
        try {
            f(false);
        } catch (IOException e) {
            System.out.println("io2=" + e.getMessage());
        } catch (Exception e) {
            System.out.println("other2=" + e.getMessage());
        }
        try {
            f(true);
        } catch (IOException | RuntimeException e) {
            System.out.println("multi=" + e.getClass().getSimpleName());
        }
    }
}
