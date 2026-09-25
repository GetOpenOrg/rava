public class EnumAdvancedTest {
    enum Planet {
        MERCURY(3.303e+23, 2.4397e6),
        VENUS(4.869e+24, 6.0518e6),
        EARTH(5.976e+24, 6.37814e6),
        MARS(6.421e+23, 3.3972e6);

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

        double surfaceWeight(double otherMass) {
            return otherMass * surfaceGravity();
        }
    }

    public static void main(String[] args) {
        // Test 1: Enum with fields and computed methods
        double earthWeight = 75.0;
        double mass = earthWeight / Planet.EARTH.surfaceGravity();
        System.out.println(String.format("%.2f", Planet.EARTH.surfaceWeight(mass)));   // 75.00
        System.out.println(String.format("%.2f", Planet.VENUS.surfaceWeight(mass)));   // 67.67
        System.out.println(String.format("%.2f", Planet.MERCURY.surfaceWeight(mass))); // 28.33

        // Test 2: Enum name and ordinal
        System.out.println(Planet.EARTH.name());      // EARTH
        System.out.println(Planet.EARTH.ordinal());   // 2
        System.out.println(Planet.MERCURY.ordinal()); // 0
        System.out.println(Planet.MARS.ordinal());    // 3

        // Test 3: Enum comparison
        Planet p1 = Planet.EARTH;
        Planet p2 = Planet.EARTH;
        Planet p3 = Planet.MARS;
        System.out.println(p1 == p2);  // true
        System.out.println(p1 == p3);  // false

        // Test 4: Switch on enum
        Planet target = Planet.MARS;
        String label;
        switch (target) {
            case MERCURY: label = "smallest"; break;
            case VENUS:   label = "hottest"; break;
            case EARTH:   label = "home"; break;
            case MARS:    label = "red"; break;
            default:      label = "unknown"; break;
        }
        System.out.println(label);  // red

        // Test 5: Multiple calls to same enum method
        System.out.println(String.format("%.2f", Planet.MARS.surfaceWeight(mass)));  // 28.40
        System.out.println(String.format("%.4f", Planet.EARTH.surfaceGravity()));   // 9.8023
    }
}
