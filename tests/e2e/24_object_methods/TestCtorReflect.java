import java.lang.reflect.Constructor;

/**
 * 构造器反射面（N1）：
 *   - 构造器不继承：getConstructors 只含本类 public 构造器，父类构造器不可经子类取得；
 *   - 可见性：非 public 构造器只出现在 getDeclaredConstructors；
 *   - Object 的 public 无参构造在反射中可见（public 面 / declared 面），可 newInstance；
 *   - 接口无构造器；抽象类有构造器但声明可见；带参构造的查找与 newInstance；
 *   - 未命中 → NoSuchMethodException。
 */
public class TestCtorReflect {
    public static class Base {
        public final int v;
        public Base() { this.v = -1; }
        public Base(int x) { this.v = x; }
    }

    public static class Child extends Base {
        public Child() { super(7); }
        Child(String s) { super(s.length()); }
    }

    public static abstract class Shape {
        public Shape() { }
        protected Shape(int sides) { }
    }

    interface Marker { }

    public static void main(String[] args) throws Exception {
        // 不继承：Child 只有自己的 public 构造器
        System.out.println("base.ctors=" + Base.class.getConstructors().length);
        System.out.println("child.ctors=" + Child.class.getConstructors().length);
        System.out.println("child.declared=" + Child.class.getDeclaredConstructors().length);
        try {
            Child.class.getConstructor(int.class);
            System.out.println("child.intCtor=unexpected");
        } catch (NoSuchMethodException e) {
            System.out.println("child.intCtor=NoSuchMethodException");
        }
        // 非 public 构造器：getConstructor 不可见、getDeclaredConstructor 可见
        try {
            Child.class.getConstructor(String.class);
            System.out.println("child.strPublic=unexpected");
        } catch (NoSuchMethodException e) {
            System.out.println("child.strPublic=NoSuchMethodException");
        }
        Constructor<Child> cs = Child.class.getDeclaredConstructor(String.class);
        System.out.println("child.strDeclared.params=" + cs.getParameterCount());

        // 带参构造：查找 + newInstance 实参传递
        Base b = Base.class.getConstructor(int.class).newInstance(42);
        System.out.println("base.newInt=" + b.v);
        Base b0 = Base.class.getConstructor().newInstance();
        System.out.println("base.newNoArg=" + b0.v);
        Child c = Child.class.getConstructor().newInstance();
        System.out.println("child.new=" + c.getClass().getSimpleName() + "," + c.v);

        // 抽象类：public 面只含 public 构造器，declared 面含 protected
        System.out.println("shape.ctors=" + Shape.class.getConstructors().length);
        System.out.println("shape.declared=" + Shape.class.getDeclaredConstructors().length);
        // 接口：无构造器
        System.out.println("iface.ctors=" + Marker.class.getConstructors().length);
        System.out.println("iface.declared=" + Marker.class.getDeclaredConstructors().length);

        // Object：public 无参构造可见、可构造
        Constructor<?>[] oc = Object.class.getConstructors();
        System.out.println("object.ctors=" + oc.length);
        System.out.println("object.params=" + oc[0].getParameterCount());
        System.out.println("object.declared=" + Object.class.getDeclaredConstructors().length);
        Object o = Object.class.getConstructor().newInstance();
        System.out.println("object.new=" + (o != null) + "," + o.getClass().getName());
        Object o2 = Object.class.getDeclaredConstructor().newInstance();
        System.out.println("object.distinct=" + (o != o2));
        try {
            Object.class.getConstructor(int.class);
            System.out.println("object.intCtor=unexpected");
        } catch (NoSuchMethodException e) {
            System.out.println("object.intCtor=NoSuchMethodException");
        }
    }
}
