public class MultiFileDemo {
    public static void main(String[] args) {
        Greeter g = new Greeter("World");
        System.out.println(g.greet());
        System.out.println(g.farewell());

        Greeter g2 = new Greeter("Java");
        System.out.println(g2.greet());
    }
}

class Greeter {
    private String name;

    public Greeter(String name) {
        this.name = name;
    }

    public String greet() {
        return "Hello, " + name + "!";
    }

    public String farewell() {
        return "Goodbye, " + name + "!";
    }
}
