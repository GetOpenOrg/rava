public class InterfaceDispatch {
    interface Greeter {
        String greet(String name);
    }

    interface Counter {
        int next();
        default int peek() { return next() - 1; }
    }

    static class HelloGreeter implements Greeter {
        public String greet(String name) { return "Hello, " + name + "!"; }
    }

    static class HiGreeter implements Greeter {
        public String greet(String name) { return "Hi, " + name + "!"; }
    }

    static class UpCounter implements Counter {
        private int val;
        public UpCounter(int start) { this.val = start; }
        public int next() { return ++val; }
    }

    public static void main(String[] args) {
        // Interface-typed variables — virtual dispatch
        Greeter g1 = new HelloGreeter();
        Greeter g2 = new HiGreeter();

        System.out.println(g1.greet("World"));  // Hello, World!
        System.out.println(g2.greet("World"));  // Hi, World!

        // instanceof through interface type
        System.out.println(g1 instanceof HelloGreeter);  // true
        System.out.println(g2 instanceof HiGreeter);     // true
        System.out.println(g1 instanceof HiGreeter);     // false

        // Counter interface
        Counter c = new UpCounter(0);
        System.out.println(c.next());  // 1
        System.out.println(c.next());  // 2
    }
}
