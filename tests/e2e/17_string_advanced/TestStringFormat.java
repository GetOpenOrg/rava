public class TestStringFormat {
    public static void main(String[] args) {
        String name = "Alice";
        int age = 30;
        double score = 98.5;

        System.out.printf("Name: %s, Age: %d%n", name, age);
        System.out.printf("Score: %.1f%n", score);
        System.out.printf("Hex: %x%n", 255);
        System.out.printf("Padded: %10s|%n", "hi");
        System.out.printf("Left: %-10s|%n", "hi");

        String formatted = String.format("(%s, %d)", name, age);
        System.out.println(formatted);
    }
}
