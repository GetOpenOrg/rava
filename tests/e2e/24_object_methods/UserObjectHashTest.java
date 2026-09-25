import java.util.HashSet;
import java.util.HashMap;
import java.util.ArrayList;

/**
 * Tests user object identity/equality in collections.
 * Verifies that HashSet/HashMap correctly handle user-defined objects
 * that don't override equals/hashCode (Java default: reference equality).
 */
public class UserObjectHashTest {

    // A simple class that does NOT override equals/hashCode
    static class Node {
        String name;
        Node(String name) { this.name = name; }
        public String toString() { return "Node(" + name + ")"; }
    }

    // A class that DOES override equals/hashCode
    static class Point {
        int x, y;
        Point(int x, int y) { this.x = x; this.y = y; }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (!(o instanceof Point)) return false;
            Point p = (Point) o;
            return x == p.x && y == p.y;
        }

        @Override
        public int hashCode() {
            return 31 * x + y;
        }

        public String toString() { return "(" + x + "," + y + ")"; }
    }

    public static void main(String[] args) {
        // Test 1: HashSet with objects that override equals/hashCode
        HashSet<Point> points = new HashSet<>();
        points.add(new Point(1, 2));
        points.add(new Point(3, 4));
        points.add(new Point(1, 2)); // duplicate by value
        System.out.println("points size: " + points.size()); // 2

        // Test 2: HashSet contains with value equality
        System.out.println("contains (1,2): " + points.contains(new Point(1, 2))); // true
        System.out.println("contains (5,6): " + points.contains(new Point(5, 6))); // false

        // Test 3: HashMap with user objects as keys
        HashMap<Point, String> map = new HashMap<>();
        map.put(new Point(1, 2), "origin");
        map.put(new Point(1, 2), "updated"); // overwrites
        System.out.println("map size: " + map.size()); // 1
        System.out.println("map get: " + map.get(new Point(1, 2))); // updated

        // Test 4: ArrayList contains (uses equals)
        ArrayList<Point> list = new ArrayList<>();
        list.add(new Point(10, 20));
        list.add(new Point(30, 40));
        System.out.println("list contains (10,20): " + list.contains(new Point(10, 20))); // true
        System.out.println("list contains (50,60): " + list.contains(new Point(50, 60))); // false

        // Test 5: Connected components pattern (same as ConnectedComponent external test)
        ArrayList<Node> nodes = new ArrayList<>();
        Node a = new Node("a");
        Node b = new Node("b");
        Node c = new Node("c");
        nodes.add(a);
        nodes.add(b);
        nodes.add(c);

        // HashSet with same references
        HashSet<Node> visited = new HashSet<>();
        visited.add(a);
        visited.add(b);
        System.out.println("visited size: " + visited.size()); // 2
        // Adding same reference again should be no-op
        boolean added = visited.add(a);
        System.out.println("add duplicate ref: " + added); // false
        System.out.println("visited size after dup: " + visited.size()); // 2
    }
}
