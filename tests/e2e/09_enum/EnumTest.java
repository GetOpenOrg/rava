public class EnumTest {

    enum Color {
        RED, GREEN, BLUE
    }

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
            final double G = 6.67300E-11;
            return G * mass / (radius * radius);
        }
    }

    public static void main(String[] args) {
        // Basic name/ordinal/toString
        Color r = Color.RED;
        System.out.println(r.name());
        System.out.println(r.ordinal());
        System.out.println(r.toString());

        Color g = Color.GREEN;
        System.out.println(g.name());
        System.out.println(g.ordinal());

        Color b = Color.BLUE;
        System.out.println(b.name());
        System.out.println(b.ordinal());

        // Switch on enum
        switch (r.ordinal()) {
            case 0:
                System.out.println("It is RED");
                break;
            case 1:
                System.out.println("It is GREEN");
                break;
            case 2:
                System.out.println("It is BLUE");
                break;
        }

        // values() iteration
        Color[] colors = Color.values();
        for (int i = 0; i < colors.length; i++) {
            System.out.println("Color: " + colors[i].name());
        }

        // Enum with constructor + custom method
        Planet earth = Planet.EARTH;
        System.out.println("Planet: " + earth.name());
        // Print surface gravity rounded to avoid floating point differences
        long sg = (long)(earth.surfaceGravity() * 100);
        System.out.println("Gravity*100: " + sg);
    }
}
