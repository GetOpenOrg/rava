public class TryResources {
    static class Resource implements AutoCloseable {
        String name;

        Resource(String name) {
            this.name = name;
            System.out.println("open: " + name);
        }

        void use() {
            System.out.println("use: " + name);
        }

        @Override
        public void close() {
            System.out.println("close: " + name);
        }
    }

    public static void main(String[] args) {
        try (Resource r = new Resource("A")) {
            r.use();
        }
        System.out.println("done");
    }
}
