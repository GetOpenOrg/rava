import java.lang.reflect.Method;
import java.lang.reflect.Modifier;

/**
 * 接口方法的反射深水：default 方法 invoke、接口 static 方法、桥方法泛型擦除签名
 * （isBridge/isDefault/isSynthetic）——spring 泛型解析与 OGNL 分派必经，e2e 此前零覆盖。
 */
public class TestInterfaceMethodReflect {

    interface Shape {
        String name();

        default String describe() {
            return "shape:" + name();
        }

        static Shape constant() {
            return new Shape() {
                @Override
                public String name() {
                    return "const";
                }
            };
        }
    }

    static class Circle implements Comparable<Circle> {
        final int r;

        Circle(int r) {
            this.r = r;
        }

        @Override
        public int compareTo(Circle o) {
            return Integer.compare(r, o.r);
        }
    }

    static class Sq implements Shape {
        @Override
        public String name() {
            return "sq";
        }
    }

    public static void main(String[] args) throws Exception {
        // default 方法：Method.invoke 驱动到接口默认实现
        Method describe = Shape.class.getMethod("describe");
        System.out.println("isDefault=" + describe.isDefault());
        System.out.println("default-invoke=" + describe.invoke(new Sq()));

        // 接口 static 方法：null 接收者
        Method constant = Shape.class.getMethod("constant");
        System.out.println("static-invoke=" + ((Shape) constant.invoke(null)).name());
        System.out.println("static-mod=" + Modifier.isStatic(constant.getModifiers()));

        // 桥方法：compareTo(Circle) 编译生成 compareTo(Object) 桥
        int bridges = 0;
        String bridgeParam = null;
        for (Method m : Circle.class.getDeclaredMethods()) {
            if (m.isBridge()) {
                bridges++;
                bridgeParam = m.getParameterTypes()[0].getSimpleName();
            }
        }
        System.out.println("bridge-count=" + bridges + " bridge-param=" + bridgeParam);

        // 桥方法的可调用性：转发到真实实现
        Method bridge = Circle.class.getMethod("compareTo", Object.class);
        System.out.println("bridge-invoke=" + bridge.invoke(new Circle(2), new Circle(1)));

        // 匿名实现的合成类形态
        Shape anon = Shape.constant();
        System.out.println("anon-synthetic=" + anon.getClass().isSynthetic()
                + " anon-name=" + anon.getClass().isAnonymousClass());
        System.out.println("anon-super=" + (anon.getClass().getSuperclass() == Object.class));
    }
}
