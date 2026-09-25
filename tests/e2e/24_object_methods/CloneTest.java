public class CloneTest {
    static class Point implements Cloneable {
        int x, y;
        Point(int x, int y) { this.x = x; this.y = y; }
        @Override
        public Point clone() {
            try { return (Point) super.clone(); }
            catch (CloneNotSupportedException e) { throw new RuntimeException(e); }
        }
        @Override
        public String toString() { return "Point(" + x + "," + y + ")"; }
    }

    public static void main(String[] args) {
        Point p1 = new Point(1, 2);
        Point p2 = p1.clone();
        p2.x = 10;
        System.out.println(p1);  // Point(1,2) - unchanged
        System.out.println(p2);  // Point(10,2)
    }
}
