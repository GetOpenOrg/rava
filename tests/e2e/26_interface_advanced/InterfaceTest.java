interface Greetable {
    String greet();
}

class English implements Greetable {
    public String greet() {
        return "Hello!";
    }
}

class Spanish implements Greetable {
    public String greet() {
        return "Hola!";
    }
}

public class InterfaceTest {
    public static void main(String[] args) {
        English eng = new English();
        Spanish spa = new Spanish();
        System.out.println(eng.greet());
        System.out.println(spa.greet());
    }
}
