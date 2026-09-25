/**
 * Month 3 测试：基础 OOP — 类实例化、字段访问、实例方法、构造函数
 */
public class SimpleClass {
    int x;
    int y;

    public SimpleClass(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int sum() {
        return this.x + this.y;
    }

    public void printInfo() {
        System.out.println(this.sum());
    }

    public static SimpleClass create(int a, int b) {
        return new SimpleClass(a, b);
    }

    public static void main(String[] args) {
        SimpleClass p = new SimpleClass(3, 4);
        System.out.println(p.x);
        System.out.println(p.y);
        System.out.println(p.sum());
        p.printInfo();

        SimpleClass q = SimpleClass.create(10, 20);
        System.out.println(q.sum());
    }
}
