
import java.lang.reflect.ParameterizedType;
import java.lang.reflect.Type;
import java.util.List;
import java.util.Map;

import org.springframework.core.GenericTypeResolver;
import org.springframework.core.ResolvableType;

/**
 * spring-core pilot C4：泛型解析 ResolvableType / GenericTypeResolver（切片 4/4）。
 */
public class SpringGenericResolverMain {

    static class Dao<T, ID> {
    }

    static class UserDao extends Dao<UserEntity, Long> {
    }

    static class UserEntity {
    }

    static class Holder {
        Map<String, List<Integer>> nested;
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        // GenericTypeResolver：父类泛型实参
        var vars = GenericTypeResolver.getTypeVariableMap(UserDao.class);
        check("typevar-t", UserEntity.class, vars.get(typeVarOf("T")));
        check("typevar-id", Long.class, vars.get(typeVarOf("ID")));

        // ResolvableType：字段泛型（Map<String, List<Integer>>）
        ResolvableType nested = ResolvableType.forField(Holder.class.getDeclaredField("nested"));
        check("map-generic", String.class, nested.getGeneric(0).resolve());
        ResolvableType listOfInt = nested.getGeneric(1);
        check("list-generic", List.class, listOfInt.resolve());
        check("list-element", Integer.class, listOfInt.getGeneric(0).resolve());

        // asMap/asCollection 判定 + 泛型串
        check("as-map", true, nested.asMap().resolve() == Map.class);
        check("generic-string", "java.util.Map<java.lang.String, java.util.List<java.lang.Integer>>",
                nested.toString());

        // 父类泛型（Super Type Token 风格）
        ResolvableType superType = ResolvableType.forClass(UserDao.class).getSuperType();
        check("super-dao", Dao.class, superType.resolve());
        check("super-generics", "[SpringGenericResolverMain$UserEntity, "
                        + "java.lang.Long]",
                java.util.Arrays.toString(superType.getGenerics()));

        // 数组与原始类型面
        ResolvableType intArray = ResolvableType.forClass(int[].class);
        check("array-component", int.class, intArray.getComponentType().resolve());
        check("is-array", true, intArray.isArray());

        // 赋值兼容判定
        check("is-assignable", true,
                ResolvableType.forClass(Long.class).isAssignableFrom(
                        ResolvableType.forClass(Long.class)));
        System.out.println("DONE SpringGenericResolverMain");
    }

    static java.lang.reflect.TypeVariable<Class<Dao>> typeVarOf(String name) {
        for (var tv : Dao.class.getTypeParameters()) {
            if (tv.getName().equals(name)) {
                return tv;
            }
        }
        throw new IllegalStateException(name);
    }
}
