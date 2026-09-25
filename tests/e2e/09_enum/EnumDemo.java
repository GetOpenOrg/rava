public class EnumDemo {
    enum Color { RED, GREEN, BLUE }

    public static void main(String[] args) {
        Color c = Color.GREEN;
        System.out.println(c.name());     // GREEN
        System.out.println(c.ordinal());  // 1
        System.out.println(Color.values().length); // 3
    }
}
