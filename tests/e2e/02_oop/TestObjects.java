public class TestObjects {
    public int x;
    public int y;

    public TestObjects(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int sum() {
        return x + y;
    }

    public void setX(int val) {
        this.x = val;
    }

    public static void main(String[] args) {
        TestP1 p = new TestP1(3, 4);
        System.out.println(p.sum());    // 7
        p.setX(10);
        System.out.println(p.sum());    // 14
        System.out.println(p.x);       // 10
    }
}
