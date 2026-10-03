import java.lang.reflect.Method;
import java.lang.reflect.ParameterizedType;
import java.lang.reflect.Type;
import java.lang.reflect.TypeVariable;
import java.lang.reflect.WildcardType;
import java.util.List;

/**
 * 类型变量与通配符边界：TypeVariable 的 getName/getBounds/getGenericDeclaration、
 * WildcardType 的上下界、泛型方法的类型参数（spring ResolvableType 的核心原料）。
 */
public class TestGenericTypeVariablesBounds {

    static class Holder<K extends Number & Comparable<K>, V> {
    }

    static <T extends List<? extends Number>> String inspect(T item) {
        return "generic";
    }

    static class Box<T> {
        T value;
    }

    public static void main(String[] args) throws Exception {
        TypeVariable<?>[] tvs = Holder.class.getTypeParameters();
        System.out.println("tv-count=" + tvs.length);
        for (TypeVariable<?> tv : tvs) {
            System.out.println("tv=" + tv.getName() + " bounds=" + tv.getBounds().length
                    + " decl=" + tv.getGenericDeclaration().getClass().getSimpleName());
        }
        // 多重边界的两个上界形态
        Type[] kBounds = tvs[0].getBounds();
        System.out.println("k-bound0=" + kBounds[0].getTypeName());
        System.out.println("k-bound1=" + kBounds[1].getTypeName());

        // 泛型方法类型参数
        Method m = TestGenericTypeVariablesBounds.class.getDeclaredMethod(
                "inspect", List.class);
        TypeVariable<?>[] mTvs = m.getTypeParameters();
        System.out.println("m-tv=" + mTvs[0].getName());
        Type bound = mTvs[0].getBounds()[0];
        System.out.println("m-bound=" + bound.getTypeName()
                + " parameterized=" + (bound instanceof ParameterizedType));
        if (bound instanceof ParameterizedType) {
            Type arg = ((ParameterizedType) bound).getActualTypeArguments()[0];
            System.out.println("m-bound-arg=" + arg.getTypeName()
                    + " wildcard=" + (arg instanceof WildcardType));
            if (arg instanceof WildcardType) {
                WildcardType w = (WildcardType) arg;
                System.out.println("upper=" + w.getUpperBounds()[0].getTypeName());
                System.out.println("lower-count=" + w.getLowerBounds().length);
            }
        }

        // super 通配符的下界形态
        Method use = UseList.class.getDeclaredMethod("use", List.class);
        Type wArg = ((ParameterizedType) use.getGenericParameterTypes()[0]).getActualTypeArguments()[0];
        if (wArg instanceof WildcardType) {
            WildcardType w = (WildcardType) wArg;
            System.out.println("super-upper=" + w.getUpperBounds()[0].getTypeName());
            System.out.println("super-lower=" + w.getLowerBounds()[0].getTypeName());
        }
    }

    static class UseList {
        void use(List<? super Integer> sink) {
        }
    }
}
