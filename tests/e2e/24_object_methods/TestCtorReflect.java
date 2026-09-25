import java.lang.reflect.Constructor;

/**
 * 构造器反射面（N1）：构造器不继承——getConstructors 只含本类 public 构造器；
 * Object 的 public 无参构造在反射中可见、可经 newInstance 构造。
 */
public class TestCtorReflect {
    public static class Base {
        public Base() { }
        public Base(int x) { }
    }

    public static class Child extends Base {
        public Child() { }
        Child(String s) { }
    }

    public static void main(String[] args) throws Exception {
        System.out.println("base.ctors=" + Base.class.getConstructors().length);
        System.out.println("child.ctors=" + Child.class.getConstructors().length);
        System.out.println("child.declared=" + Child.class.getDeclaredConstructors().length);

        Constructor<?>[] oc = Object.class.getConstructors();
        System.out.println("object.ctors=" + oc.length);
        System.out.println("object.params=" + oc[0].getParameterCount());
        Object o = Object.class.getConstructor().newInstance();
        System.out.println("object.new=" + (o != null) + "," + o.getClass().getName());

        Child c = Child.class.getConstructor().newInstance();
        System.out.println("child.new=" + c.getClass().getSimpleName());
    }
}
