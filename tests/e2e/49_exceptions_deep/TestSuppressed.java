public class TestSuppressed {
    static class CloseFail implements AutoCloseable {
        public void close() { throw new IllegalStateException("close-fail"); }
    }

    static class BodyFail implements AutoCloseable {
        public void close() {}
        void run() { throw new RuntimeException("body-fail"); }
    }

    public static void main(String[] args) {
        try (CloseFail c = new CloseFail()) {
            throw new RuntimeException("body");
        } catch (Exception e) {
            System.out.println("primary=" + e.getMessage());
            Throwable[] supp = e.getSuppressed();
            System.out.println("suppCount=" + supp.length);
            for (Throwable t : supp) System.out.println("supp=" + t.getMessage());
        }
        try (BodyFail b = new BodyFail()) {
            b.run();
        } catch (Exception e) {
            System.out.println("bodyOnly=" + e.getMessage());
            System.out.println("suppCount2=" + e.getSuppressed().length);
        }
    }
}
