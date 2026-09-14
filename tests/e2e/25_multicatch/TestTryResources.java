public class TestTryResources {

    static class Resource implements AutoCloseable {
        String name;
        boolean closed = false;

        Resource(String name) {
            this.name = name;
            System.out.println("open: " + name);
        }

        void use() {
            System.out.println("use: " + name);
        }

        @Override
        public void close() {
            closed = true;
            System.out.println("close: " + name);
        }
    }

    static class FailingResource implements AutoCloseable {
        FailingResource() { System.out.println("open: failing"); }

        void use() {
            throw new RuntimeException("use failed");
        }

        @Override
        public void close() {
            System.out.println("close: failing");
        }
    }

    static void basic() {
        try (Resource r = new Resource("R1")) {
            r.use();
        }
        // open:R1, use:R1, close:R1
    }

    static void multipleResources() {
        try (Resource r1 = new Resource("R1");
             Resource r2 = new Resource("R2")) {
            r1.use();
            r2.use();
        }
        // open:R1, open:R2, use:R1, use:R2, close:R2, close:R1 (reverse order)
    }

    static void withException() {
        try (FailingResource fr = new FailingResource()) {
            fr.use();
        } catch (RuntimeException e) {
            System.out.println("caught: " + e.getMessage());
        }
        // open:failing, close:failing, caught:use failed
    }

    public static void main(String[] args) {
        basic();
        System.out.println("---");
        multipleResources();
        System.out.println("---");
        withException();
    }
}
