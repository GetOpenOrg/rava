public class InterfaceDefaultMethod {
    interface Greeter {
        String name();
        default String greet() {
            return "Hello, " + name() + "!";
        }
        default int nameLength() {
            return name().length();
        }
    }

    static class EnglishGreeter implements Greeter {
        private String personName;
        EnglishGreeter(String n) { this.personName = n; }
        public String name() { return personName; }
    }

    static class FrenchGreeter implements Greeter {
        private String personName;
        FrenchGreeter(String n) { this.personName = n; }
        public String name() { return personName; }
        // Overrides the default greet() method
        public String greet() { return "Bonjour, " + personName + "!"; }
    }

    public static void main(String[] args) {
        Greeter g1 = new EnglishGreeter("Alice");
        Greeter g2 = new FrenchGreeter("Bob");
        System.out.println(g1.greet());     // Uses default: Hello, Alice!
        System.out.println(g1.nameLength()); // Uses default: 5
        System.out.println(g2.greet());     // Overridden: Bonjour, Bob!
        System.out.println(g2.nameLength()); // Uses default: 3
    }
}
