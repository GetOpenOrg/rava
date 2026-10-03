import java.lang.reflect.GenericArrayType;
import java.lang.reflect.Method;
import java.lang.reflect.ParameterizedType;
import java.lang.reflect.Type;
import java.lang.reflect.TypeVariable;
import java.util.List;
import java.util.Map;

/**
 * 泛型元数据深水补全（方法级实测：getGenericReturnType 15 jar /
 * GenericArrayType.getGenericComponentType 16 jar，此前零覆盖）：
 * 方法返回泛型、参数泛型、T[] 组件类型解析（TypeVariable / ParameterizedType 递归）。
 */
public class TestGenericTypesDeep {

    static class Repo<T> {
        public T find(Long id) {
            return null;
        }

        public List<T> findAll() {
            return null;
        }

        public Map<String, List<T>> group() {
            return null;
        }

        public T[] batch(T[] in) {
            return in;
        }
    }

    static class UserRepo extends Repo<String> {
    }

    interface Sink<T extends Number> {
        void accept(T[] items);
    }

    public static void main(String[] args) throws Exception {
        Method find = Repo.class.getMethod("find", Long.class);
        System.out.println("find-return=" + find.getGenericReturnType().getTypeName());

        Method findAll = Repo.class.getMethod("findAll");
        Type rt = findAll.getGenericReturnType();
        System.out.println("list-return=" + rt.getTypeName()
                + " parameterized=" + (rt instanceof ParameterizedType));
        System.out.println("list-arg=" + ((ParameterizedType) rt).getActualTypeArguments()[0].getTypeName());

        // 嵌套泛型：Map<String, List<T>>
        Method group = Repo.class.getMethod("group");
        ParameterizedType mapType = (ParameterizedType) group.getGenericReturnType();
        System.out.println("map-raw=" + ((Class<?>) mapType.getRawType()).getSimpleName());
        Type listArg = mapType.getActualTypeArguments()[1];
        System.out.println("nested=" + listArg.getTypeName()
                + " inner=" + ((ParameterizedType) listArg).getActualTypeArguments()[0].getTypeName());

        // 泛型数组：T[] → GenericArrayType，组件是 TypeVariable
        Method batch = Repo.class.getMethod("batch", Object[].class);
        Type bt = batch.getGenericReturnType();
        System.out.println("array-return=" + bt.getTypeName() + " gat=" + (bt instanceof GenericArrayType));
        if (bt instanceof GenericArrayType gat) {
            Type comp = gat.getGenericComponentType();
            System.out.println("component=" + comp.getTypeName()
                    + " is-typevar=" + (comp instanceof TypeVariable));
        }

        // 参数侧同族
        System.out.println("batch-param=" + batch.getGenericParameterTypes()[0].getTypeName());
        Method accept = Sink.class.getMethod("accept", Number[].class);
        Type p = accept.getGenericParameterTypes()[0];
        System.out.println("sink-param=" + p.getTypeName());
        if (p instanceof GenericArrayType g2) {
            System.out.println("sink-component=" + g2.getGenericComponentType().getTypeName());
        }
    }
}
