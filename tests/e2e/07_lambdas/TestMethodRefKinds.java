import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.BiFunction;
import java.util.function.Function;
import java.util.function.IntFunction;
import java.util.function.Supplier;

class Person {
    String name;
    int age;

    Person() {
        this("anonymous", 0);
    }

    Person(String name) {
        this(name, 0);
    }

    Person(String name, int age) {
        this.name = name;
        this.age = age;
    }

    static Person of(String name) {
        return new Person(name, 1);
    }

    String greet(String prefix) {
        return prefix + " " + name;
    }

    int getAge() {
        return age;
    }

    @Override
    public String toString() {
        return name + "/" + age;
    }
}

class SubPerson extends Person {
    SubPerson(String name) {
        super(name, 9);
    }

    String subOnly() {
        return "sub:" + name;
    }
}

public class TestMethodRefKinds {

    public static void main(String[] args) {
        // 1. 静态方法引用
        Function<String, Person> staticRef = Person::of;
        System.out.println(staticRef.apply("alice"));

        // 2. 构造器引用（无参）
        Supplier<Person> ctor0 = Person::new;
        System.out.println("ctor0=" + ctor0.get());

        // 3. 构造器引用（单参）
        Function<String, Person> ctor1 = Person::new;
        System.out.println("ctor1=" + ctor1.apply("bob"));

        // 4. 构造器引用（子类）
        Function<String, SubPerson> ctorSub = SubPerson::new;
        System.out.println("ctorSub=" + ctorSub.apply("carol"));

        // 5. 绑定实例方法引用
        Person p = new Person("dave", 40);
        Supplier<Integer> bound = p::getAge;
        Function<String, String> boundArgs = p::greet;
        System.out.println("bound age=" + bound.get());
        System.out.println("bound greet=" + boundArgs.apply("hi"));

        // 6. 未绑定实例方法引用：实例作为第一个参数
        Function<Person, Integer> unbound = Person::getAge;
        BiFunction<Person, String, String> unboundArgs = Person::greet;
        System.out.println("unbound age=" + unbound.apply(p));
        System.out.println("unbound greet=" + unboundArgs.apply(p, "hey"));

        // 7. 数组构造器引用
        IntFunction<String[]> arrayCtor = String[]::new;
        String[] arr = arrayCtor.apply(3);
        arr[0] = "a";
        arr[1] = "b";
        arr[2] = "c";
        System.out.println("arrayCtor len=" + arr.length + " " + Arrays.toString(arr));

        IntFunction<int[]> intArrayCtor = int[]::new;
        int[] ia = intArrayCtor.apply(2);
        ia[1] = 7;
        System.out.println("intArray=" + Arrays.toString(ia));

        // 8. 父类方法引用 super::method（在实例方法中）
        Supplier<String> superRef = new Object() {
            Supplier<String> make() {
                return this::toString;
            }
        }.make();
        System.out.println("superRef produced=" + (superRef.get() != null));

        // 9. 方法引用配合 stream / list
        List<String> names = new ArrayList<>(Arrays.asList("x", "yy", "zzz"));
        names.forEach(System.out::println);
        StringBuilder sb = new StringBuilder();
        names.forEach(sb::append);
        System.out.println("appended=" + sb);

        // 10. 方法引用作为返回值
        System.out.println(makeTransformer().apply("upper"));

        System.out.println("done");
    }

    static Function<String, String> makeTransformer() {
        Person host = new Person("host", 1);
        return host::greet;
    }
}
