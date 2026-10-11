// 接口重新声明 Object 方法（equals / hashCode / toString）且 equals 带同名重载：
// 经接口引用与经实现类调用两种重载，Rust 方法名须在声明与调用点一致
public class TestIfaceObjectOverload {
    interface Named {
        boolean equals(Named other);
        boolean equals(Object other);
        int hashCode();
        String toString();
        String id();
    }

    static final class SimpleName implements Named {
        private final String id;
        SimpleName(String id) { this.id = id; }
        public String id() { return id; }
        public boolean equals(Named other) { return other != null && id.equals(other.id()); }
        public boolean equals(Object other) { return other instanceof Named && equals((Named) other); }
        public int hashCode() { return id.hashCode(); }
        public String toString() { return "SimpleName(" + id + ")"; }
    }

    static final class UpperName implements Named {
        private final String id;
        UpperName(String id) { this.id = id.toUpperCase(); }
        public String id() { return id; }
        public boolean equals(Named other) { return other != null && id.equalsIgnoreCase(other.id()); }
        public boolean equals(Object other) { return other instanceof Named && equals((Named) other); }
        public int hashCode() { return id.length(); }
        public String toString() { return "UpperName(" + id + ")"; }
    }

    static boolean viaIfaceNamed(Named a, Named b) { return a.equals(b); }
    static boolean viaIfaceObject(Named a, Object b) { return a.equals(b); }

    public static void main(String[] args) {
        Named a = new SimpleName("alice");
        Named b = new UpperName("alice");
        Named c = new SimpleName("bob");
        SimpleName d = new SimpleName("alice");

        System.out.println("iface named a=b: " + viaIfaceNamed(a, b));
        System.out.println("iface named a=c: " + viaIfaceNamed(a, c));
        System.out.println("iface object a=d: " + viaIfaceObject(a, d));
        System.out.println("iface object a=str: " + viaIfaceObject(a, "alice"));
        System.out.println("iface object b=a: " + b.equals((Object) a));
        System.out.println("iface hash a: " + a.hashCode());
        System.out.println("iface hash b: " + b.hashCode());
        System.out.println("iface str: " + a.toString() + " " + b);

        System.out.println("class named d=a: " + d.equals(a));
        System.out.println("class object d=b: " + d.equals((Object) b));
        System.out.println("class object d=null: " + d.equals((Object) null));
        System.out.println("class hash d: " + d.hashCode());
        System.out.println("class str d: " + d.toString());

        Object o = a;
        System.out.println("object equals: " + o.equals(d) + " " + o.hashCode() + " " + o);
    }
}
