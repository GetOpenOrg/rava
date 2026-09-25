public class EnumSwitchTest {
    enum Season { SPRING, SUMMER, AUTUMN, WINTER }

    enum Planet {
        MERCURY(3.303e+23, 2.4397e6),
        VENUS(4.869e+24, 6.0518e6),
        EARTH(5.976e+24, 6.37814e6);

        private final double mass;
        private final double radius;

        Planet(double mass, double radius) {
            this.mass = mass;
            this.radius = radius;
        }

        double surfaceGravity() {
            return 6.67300E-11 * mass / (radius * radius);
        }
    }

    public static void main(String[] args) {
        // Test 1: Switch on enum
        Season s = Season.SUMMER;
        switch (s) {
            case SPRING: System.out.println("Spring"); break;
            case SUMMER: System.out.println("Summer"); break;
            case AUTUMN: System.out.println("Autumn"); break;
            case WINTER: System.out.println("Winter"); break;
        }

        // Test 2: name() and ordinal()
        System.out.println(Season.SPRING.name());
        System.out.println(Season.SPRING.ordinal());
        System.out.println(Season.WINTER.ordinal());

        // Test 3: values() iteration
        int count = 0;
        for (Season season : Season.values()) {
            count++;
        }
        System.out.println(count);

        // Test 4: Enum with fields and methods
        System.out.println(Planet.EARTH.name());
        double g = Planet.EARTH.surfaceGravity();
        System.out.println(g > 9.0 && g < 10.0);

        // Test 5: Enum compareTo
        System.out.println(Season.SPRING.compareTo(Season.WINTER) < 0);
        System.out.println(Season.WINTER.compareTo(Season.SPRING) > 0);

        // Test 6: Enum toString
        System.out.println(Season.AUTUMN.toString());

        // Test 7: Switch with default
        Season s2 = Season.SPRING;
        String result;
        switch (s2) {
            case SUMMER: result = "Hot"; break;
            case WINTER: result = "Cold"; break;
            default: result = "Mild"; break;
        }
        System.out.println(result);

        // Test 8: Enum equality
        Season a = Season.SPRING;
        Season b = Season.SPRING;
        System.out.println(a == b);

        // Test 9: Multiple switches with fall-through grouping
        for (Season season : Season.values()) {
            switch (season) {
                case SPRING: case SUMMER:
                    System.out.print("Warm ");
                    break;
                case AUTUMN: case WINTER:
                    System.out.print("Cool ");
                    break;
            }
        }
        System.out.println();
    }
}
