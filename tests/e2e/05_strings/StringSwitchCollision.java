/**
 * Tests String switch with hash collisions.
 * "Aa" and "BB" have the same hashCode (2112).
 * The compiler must generate two equals() checks in the same switch arm.
 */
public class StringSwitchCollision {
    static String describe(String s) {
        switch (s) {
            case "Aa": return "got Aa";
            case "BB": return "got BB";
            case "Cc": return "got Cc";
            default:   return "other";
        }
    }

    public static void main(String[] args) {
        System.out.println(describe("Aa"));    // got Aa
        System.out.println(describe("BB"));    // got BB
        System.out.println(describe("Cc"));    // got Cc
        System.out.println(describe("other")); // other
        System.out.println(describe("AA"));    // other (not matching)
        System.out.println(describe("bb"));    // other (not matching)
    }
}
