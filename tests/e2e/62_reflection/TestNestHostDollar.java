// getNestHost 读 NestHost / NestMembers 元数据：名字含 `$` 的顶层类自成 nest，真正的嵌套类归外层
public class TestNestHostDollar {
    static class Inner {
        class Deep {}
    }

    interface Api {}

    public static void main(String[] args) {
        System.out.println(TestNestHostDollar$Top.class.getNestHost().getName());
        System.out.println(TestNestHostDollar$Top.class.isNestmateOf(TestNestHostDollar.class));
        System.out.println(TestNestHostDollar$Top.class.getNestMembers().length);
        System.out.println(Inner.class.getNestHost().getName());
        System.out.println(Inner.Deep.class.getNestHost().getName());
        System.out.println(Inner.Deep.class.isNestmateOf(Api.class));
        System.out.println(TestNestHostDollar.class.getNestHost().getName());
        System.out.println(int.class.getNestHost().getName());
        System.out.println(String[].class.getNestHost().getName());
        Runnable r = () -> {};
        System.out.println(r.getClass().getNestHost().getName());
        java.util.Set<String> names = new java.util.TreeSet<>();
        for (Class<?> c : TestNestHostDollar.class.getNestMembers()) names.add(c.getName());
        System.out.println(names);
    }
}

class TestNestHostDollar$Top {
}
