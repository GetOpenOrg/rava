import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.Set;

import org.springframework.core.GenericTypeResolver;
import org.springframework.core.MethodParameter;
import org.springframework.core.ResolvableType;

/**
 * spring-core 切片 sc1（矩阵 #11）：泛型元数据反射。
 *
 * 覆盖面：ResolvableType（forClass / forField / forMethodReturnType / forMethodParameter /
 * forClassWithGenerics / as / getGeneric / resolveGenerics / isAssignableFrom / toString）、
 * MethodParameter（嵌套层级、Optional 展开、getGenericParameterType、withContainingClass）、
 * GenericTypeResolver（resolveTypeArgument(s) / resolveReturnType / resolveParameterType）。
 * 泛型信息全部来自类文件 Signature 属性（Class.getGenericSuperclass / Field.getGenericType /
 * Method.getGenericReturnType 等），forField 等经 SerializableTypeWrapper 的动态代理包装。
 */
public class SpringResolvableTypeMain {

    interface Repository<T, ID> {
        T findById(ID id);
    }

    static class User {
    }

    static class UserRepository implements Repository<User, Long> {
        public User findById(Long id) {
            return null;
        }
    }

    static abstract class Base<E> {
        public E value;

        public E get() {
            return value;
        }

        public void put(E e) {
            value = e;
        }
    }

    static class StringBase extends Base<String> {
    }

    static class Holder {
        public Map<String, List<Integer>> index;
        public List<Set<String>>[] buckets;
        public Optional<Map<Long, String>> maybe;
        public HashMap<String, Integer> counts;

        public Map<String, List<Integer>> lookup(List<? extends Number> nums, Optional<String> name) {
            return null;
        }

        public <K extends Comparable<K>> K pick(K[] items) {
            return null;
        }
    }

    static void show(String label, Object value) {
        System.out.println(label + " = " + value);
    }

    static String names(Class<?>[] classes) {
        if (classes == null) {
            return "null";
        }
        String[] parts = new String[classes.length];
        for (int i = 0; i < classes.length; i++) {
            parts[i] = classes[i] == null ? "null" : classes[i].getSimpleName();
        }
        return Arrays.toString(parts);
    }

    public static void main(String[] args) throws Exception {
        // ── ResolvableType：字段泛型 ──
        Field index = Holder.class.getField("index");
        ResolvableType indexType = ResolvableType.forField(index);
        show("index", indexType);
        show("index.raw", indexType.resolve().getSimpleName());
        show("index.generic0", indexType.getGeneric(0));
        show("index.generic1", indexType.getGeneric(1));
        show("index.generic1.0", indexType.getGeneric(1, 0).resolve().getSimpleName());
        show("index.asMap.key", indexType.asMap().getGeneric(0).resolve().getSimpleName());
        show("index.hasGenerics", indexType.hasGenerics());
        show("index.hasUnresolvable", indexType.hasUnresolvableGenerics());

        ResolvableType buckets = ResolvableType.forField(Holder.class.getField("buckets"));
        show("buckets", buckets);
        show("buckets.isArray", buckets.isArray());
        show("buckets.component", buckets.getComponentType());
        show("buckets.component.generic0.generic0", buckets.getComponentType().getGeneric(0).getGeneric(0));

        ResolvableType counts = ResolvableType.forField(Holder.class.getField("counts"));
        show("counts.asMap", counts.asMap());
        show("counts.superType", counts.getSuperType());
        show("counts.resolveGenerics", names(counts.asMap().resolveGenerics()));

        // ── 接口与父类泛型实参 ──
        ResolvableType repo = ResolvableType.forClass(UserRepository.class).as(Repository.class);
        show("repo.as", repo);
        show("repo.generics", names(repo.resolveGenerics()));
        ResolvableType sb = ResolvableType.forClass(StringBase.class);
        show("stringBase.super", sb.getSuperType());
        show("stringBase.super.generic", sb.getSuperType().resolveGeneric(0).getSimpleName());

        // ── 方法返回值 / 参数 ──
        Method lookup = Holder.class.getMethod("lookup", List.class, Optional.class);
        show("lookup.return", ResolvableType.forMethodReturnType(lookup));
        show("lookup.param0", ResolvableType.forMethodParameter(lookup, 0));
        show("lookup.param1", ResolvableType.forMethodParameter(lookup, 1));
        Method get = Base.class.getMethod("get");
        show("base.get.return(StringBase)",
                ResolvableType.forMethodReturnType(get, StringBase.class).resolve().getSimpleName());

        // ── 构造与可赋值判定 ──
        ResolvableType listOfString = ResolvableType.forClassWithGenerics(List.class, String.class);
        ResolvableType listOfInteger = ResolvableType.forClassWithGenerics(List.class, Integer.class);
        ResolvableType mapType = ResolvableType.forClassWithGenerics(Map.class,
                ResolvableType.forClass(String.class), listOfInteger);
        show("listOfString", listOfString);
        show("mapType", mapType);
        show("mapType.assignableFrom(index)", mapType.isAssignableFrom(indexType));
        show("listOfString.assignableFrom(listOfInteger)", listOfString.isAssignableFrom(listOfInteger));
        show("raw List assignableFrom listOfString",
                ResolvableType.forClass(List.class).isAssignableFrom(listOfString));
        show("listOfString.equals(same)",
                listOfString.equals(ResolvableType.forClassWithGenerics(List.class, String.class)));
        show("forInstance", ResolvableType.forInstance(new StringBase()).getSuperType());

        // ── MethodParameter ──
        MethodParameter p0 = new MethodParameter(lookup, 0);
        show("mp0.type", p0.getParameterType().getSimpleName());
        show("mp0.generic", p0.getGenericParameterType());
        show("mp0.index", p0.getParameterIndex());
        show("mp0.declaring", p0.getDeclaringClass().getSimpleName());
        MethodParameter p0n = p0.nested();
        show("mp0.nested.level", p0n.getNestingLevel());
        show("mp0.nested.type", p0n.getNestedParameterType().getSimpleName());
        MethodParameter p1 = new MethodParameter(lookup, 1);
        show("mp1.isOptional", p1.isOptional());
        show("mp1.nestedIfOptional.type", p1.nestedIfOptional().getNestedParameterType().getSimpleName());
        MethodParameter ret = new MethodParameter(lookup, -1);
        show("mpRet.type", ret.getParameterType().getSimpleName());
        show("mpRet.generic", ret.getGenericParameterType());
        Method put = Base.class.getMethod("put", Object.class);
        MethodParameter putP = new MethodParameter(put, 0).withContainingClass(StringBase.class);
        show("put.containing", putP.getContainingClass().getSimpleName());
        show("put.resolved", ResolvableType.forMethodParameter(putP).resolve().getSimpleName());
        show("put.resolveParameterType",
                GenericTypeResolver.resolveParameterType(new MethodParameter(put, 0), StringBase.class).getSimpleName());
        Method pick = Holder.class.getMethod("pick", Comparable[].class);
        show("pick.generic", new MethodParameter(pick, 0).getGenericParameterType());
        show("pick.return.resolve", ResolvableType.forMethodReturnType(pick).resolve().getSimpleName());

        // ── GenericTypeResolver ──
        show("gtr.typeArgs(UserRepository, Repository)",
                names(GenericTypeResolver.resolveTypeArguments(UserRepository.class, Repository.class)));
        show("gtr.typeArg(StringBase, Base)",
                GenericTypeResolver.resolveTypeArgument(StringBase.class, Base.class).getSimpleName());
        show("gtr.returnType(get, StringBase)",
                GenericTypeResolver.resolveReturnType(get, StringBase.class).getSimpleName());
        show("gtr.typeArgs(String, Comparable)",
                names(GenericTypeResolver.resolveTypeArguments(String.class, Comparable.class)));
        System.out.println("done");
    }
}
