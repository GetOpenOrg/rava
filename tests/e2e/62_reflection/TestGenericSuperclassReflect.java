import java.lang.reflect.GenericArrayType;
import java.lang.reflect.ParameterizedType;
import java.lang.reflect.Type;
import java.util.List;

/**
 * 参数化超类 / 接口的泛型元数据反射（e2e 能力补齐）。
 *
 * getGenericSuperclass / getActualTypeArguments / getGenericInterfaces 是
 * hamcrest TypeSafeMatcher（ReflectiveTypeFinder）与 spring ResolvableType
 * 的共同地基，此前 e2e 零覆盖。
 */
public class TestGenericSuperclassReflect {

    abstract static class Box<T> {
    }

    static class StringBox extends Box<String> {
    }

    static class IntBox extends Box<Integer> {
    }

    @SuppressWarnings("rawtypes")
    static class RawBox extends Box {
    }

    interface Sink<T> {
        void put(T t);
    }

    static class StringSink implements Sink<String> {
        public void put(String t) {
        }
    }

    static class ListSink implements Sink<List<String>> {
        public void put(List<String> t) {
        }
    }

    static class ArrayHolder extends Box<String[]> {
    }

    public static void main(String[] args) {
        Type sup = StringBox.class.getGenericSuperclass();
        System.out.println("sup-parameterized=" + (sup instanceof ParameterizedType));
        ParameterizedType pt = (ParameterizedType) sup;
        System.out.println("raw=" + pt.getRawType().getTypeName());
        for (Type a : pt.getActualTypeArguments()) {
            System.out.println("arg=" + a.getTypeName() + " isClass=" + (a instanceof Class));
        }

        ParameterizedType pt2 = (ParameterizedType) IntBox.class.getGenericSuperclass();
        System.out.println("int-arg=" + pt2.getActualTypeArguments()[0].getTypeName());

        // raw 继承：getGenericSuperclass 退回 Class 本身，非参数化类型
        Type raw = RawBox.class.getGenericSuperclass();
        System.out.println("raw-super=" + raw.getTypeName()
                + " parameterized=" + (raw instanceof ParameterizedType));

        for (Type itf : StringSink.class.getGenericInterfaces()) {
            System.out.println("itf=" + itf.getTypeName());
        }
        for (Type itf : ListSink.class.getGenericInterfaces()) {
            if (itf instanceof ParameterizedType) {
                ParameterizedType p = (ParameterizedType) itf;
                System.out.println("listSink-raw=" + p.getRawType().getTypeName()
                        + " arg=" + p.getActualTypeArguments()[0].getTypeName());
            }
        }

        // 泛型数组类型实参
        Type arr = ((ParameterizedType) ArrayHolder.class.getGenericSuperclass())
                .getActualTypeArguments()[0];
        System.out.println("array-arg=" + arr.getTypeName()
                + " gat=" + (arr instanceof GenericArrayType));

        // Object 盒顶：getGenericSuperclass 为 null
        System.out.println("object-super-null=" + (Object.class.getGenericSuperclass() == null));
    }
}
