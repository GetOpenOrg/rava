public class TryResourcesException {
    static class Resource implements AutoCloseable {
        String name;

        Resource(String name) {
            this.name = name;
            System.out.println("open: " + name);
        }

        void useAndThrow() {
            System.out.println("use: " + name);
            throw new RuntimeException("body error");
        }

        @Override
        public void close() {
            System.out.println("close: " + name);
        }
    }

    public static void main(String[] args) {
        // Test 1: happy path (no exception)
        try (Resource r = new Resource("A")) {
            System.out.println("using: " + r.name);
        }
        System.out.println("after A");

        // Test 2: body throws — close() must still be called
        try {
            try (Resource r = new Resource("B")) {
                r.useAndThrow();
            }
        } catch (RuntimeException e) {
            System.out.println("caught: " + e.getMessage());
        }
        System.out.println("done");
    }
}
