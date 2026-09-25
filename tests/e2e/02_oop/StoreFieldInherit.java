public class StoreFieldInherit {
    static class Base {
        int value;
        String name;
        Base(int v, String n) { this.value = v; this.name = n; }
    }
    static class Child extends Base {
        int extra;
        Child(int v, String n, int e) {
            super(v, n);
            this.extra = e;
        }
        void setValue(int v) { this.value = v; }
        void setName(String n) { this.name = n; }
    }
    public static void main(String[] args) {
        Child c = new Child(10, "hello", 99);
        System.out.println(c.value);   // 10
        System.out.println(c.name);    // hello
        System.out.println(c.extra);   // 99
        c.setValue(42);
        c.setName("world");
        System.out.println(c.value);   // 42
        System.out.println(c.name);    // world
    }
}
