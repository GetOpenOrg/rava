public class AnonymousClass2 {
    interface Printer {
        void print(String msg);
    }

    static Printer makePrinter(String prefix) {
        return new Printer() {
            public void print(String msg) {
                System.out.println(prefix + ": " + msg);
            }
        };
    }

    public static void main(String[] args) {
        Printer p = makePrinter("INFO");
        p.print("Hello");  // INFO: Hello
        p.print("World");  // INFO: World
    }
}
