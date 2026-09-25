public class NestedClasses {
    static class Point {
        int x, y;
        
        Point(int x, int y) {
            this.x = x;
            this.y = y;
        }
        
        int distanceSquared(Point other) {
            int dx = this.x - other.x;
            int dy = this.y - other.y;
            return dx * dx + dy * dy;
        }
        
        @Override
        public String toString() {
            return "(" + x + "," + y + ")";
        }
    }
    
    static class Rectangle {
        Point topLeft;
        Point bottomRight;
        
        Rectangle(int x1, int y1, int x2, int y2) {
            this.topLeft = new Point(x1, y1);
            this.bottomRight = new Point(x2, y2);
        }
        
        int area() {
            int w = bottomRight.x - topLeft.x;
            int h = bottomRight.y - topLeft.y;
            return w * h;
        }
        
        boolean contains(Point p) {
            if (p.x < topLeft.x) return false;
            if (p.x > bottomRight.x) return false;
            if (p.y < topLeft.y) return false;
            if (p.y > bottomRight.y) return false;
            return true;
        }
    }
    
    static class Builder {
        private int value = 0;
        
        Builder add(int n) {
            value += n;
            return this;
        }
        
        Builder multiply(int n) {
            value *= n;
            return this;
        }
        
        int build() {
            return value;
        }
    }
    
    public static void main(String[] args) {
        Point p1 = new Point(0, 0);
        Point p2 = new Point(3, 4);
        System.out.println(p1.distanceSquared(p2));  // 25
        System.out.println(p2.toString());            // (3,4)
        
        Rectangle r = new Rectangle(0, 0, 10, 5);
        System.out.println(r.area());                 // 50
        System.out.println(r.contains(new Point(5, 3)));  // true
        System.out.println(r.contains(new Point(11, 3))); // false
        
        int result = new Builder().add(5).multiply(3).add(2).build();
        System.out.println(result);                   // 17
    }
}
