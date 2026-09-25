import java.util.ArrayList;
import java.util.List;
import java.util.HashMap;
import java.util.Map;

// Multi-class: composition pattern + shared methods + collections
class Point {
    int x, y;

    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    double distanceTo(Point other) {
        int dx = this.x - other.x;
        int dy = this.y - other.y;
        return Math.sqrt(dx * dx + dy * dy);
    }

    String toStr() {
        return "(" + x + "," + y + ")";
    }
}

class LineSegment {
    Point start, end;

    LineSegment(Point start, Point end) {
        this.start = start;
        this.end = end;
    }

    double length() {
        return start.distanceTo(end);
    }

    String toStr() {
        return start.toStr() + "->" + end.toStr();
    }
}

class Polygon {
    List<Point> vertices;
    String name;

    Polygon(String name) {
        this.name = name;
        this.vertices = new ArrayList<>();
    }

    void addVertex(Point p) {
        vertices.add(p);
    }

    double perimeter() {
        double total = 0;
        int n = vertices.size();
        for (int i = 0; i < n; i++) {
            Point a = vertices.get(i);
            Point b = vertices.get((i + 1) % n);
            total += a.distanceTo(b);
        }
        return total;
    }

    String toStr() {
        StringBuilder sb = new StringBuilder(name + "[");
        for (int i = 0; i < vertices.size(); i++) {
            if (i > 0) sb.append(",");
            sb.append(vertices.get(i).toStr());
        }
        sb.append("]");
        return sb.toString();
    }
}

public class InheritancePolyTest {
    public static void main(String[] args) {
        // Points
        Point p1 = new Point(0, 0);
        Point p2 = new Point(3, 4);
        Point p3 = new Point(6, 0);

        System.out.println("p1: " + p1.toStr());
        System.out.println("p2: " + p2.toStr());
        System.out.println("dist p1-p2: " + String.format("%.2f", p1.distanceTo(p2)));

        // Line segment
        LineSegment line = new LineSegment(p1, p2);
        System.out.println("line: " + line.toStr());
        System.out.println("length: " + String.format("%.2f", line.length()));

        // Triangle
        Polygon tri = new Polygon("Triangle");
        tri.addVertex(p1);
        tri.addVertex(p2);
        tri.addVertex(p3);
        System.out.println("tri: " + tri.toStr());
        System.out.println("perimeter: " + String.format("%.2f", tri.perimeter()));

        // Square
        Polygon sq = new Polygon("Square");
        sq.addVertex(new Point(0, 0));
        sq.addVertex(new Point(5, 0));
        sq.addVertex(new Point(5, 5));
        sq.addVertex(new Point(0, 5));
        System.out.println("sq: " + sq.toStr());
        System.out.println("sq perimeter: " + String.format("%.2f", sq.perimeter()));

        // Map of polygons
        Map<String, Polygon> shapes = new HashMap<>();
        shapes.put("tri", tri);
        shapes.put("sq", sq);
        Polygon found = shapes.get("tri");
        System.out.println("found: " + found.toStr());

        // List of points
        List<Point> points = new ArrayList<>();
        points.add(p1);
        points.add(p2);
        points.add(p3);
        System.out.println("points: " + points.size());

        System.out.println("Done.");
    }
}
