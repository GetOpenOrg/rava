class BaseInit {
    static { System.out.println("Base static block"); }
    static String baseStaticField = echo("Base static field");
    { System.out.println("Base instance block"); }
    String baseField = echo("Base instance field");

    BaseInit() {
        System.out.println("Base constructor body");
    }

    static String echo(String s) {
        System.out.println(s);
        return s;
    }
}

class DerivedInit extends BaseInit {
    static { System.out.println("Derived static block"); }
    static String derivedStaticField = BaseInit.echo("Derived static field");
    { System.out.println("Derived instance block"); }
    String derivedField = BaseInit.echo("Derived instance field");

    DerivedInit() {
        System.out.println("Derived constructor body");
    }
}

class InitCounter {
    static int created;
    int id;
    InitCounter() {
        id = ++created;
        System.out.println("created id=" + id + " total=" + created);
    }
}

public class TestInitOrder {

    static { System.out.println("Test static block 1"); }
    static String f1 = BaseInit.echo("Test static field 1");
    { System.out.println("Test instance block 1"); }
    String i1 = BaseInit.echo("Test instance field 1");
    static String f2 = BaseInit.echo("Test static field 2");
    static { System.out.println("Test static block 2"); }
    { System.out.println("Test instance block 2"); }
    String i2 = BaseInit.echo("Test instance field 2");

    TestInitOrder() {
        System.out.println("Test constructor body");
    }

    public static void main(String[] args) {
        System.out.println("--- new DerivedInit() ---");
        DerivedInit d = new DerivedInit();
        System.out.println("derivedField=" + d.derivedField + " baseField=" + d.baseField);

        System.out.println("--- new DerivedInit() again (static 只跑一次) ---");
        DerivedInit d2 = new DerivedInit();

        System.out.println("--- new TestInitOrder() ---");
        TestInitOrder t = new TestInitOrder();
        System.out.println("i1=" + t.i1 + " i2=" + t.i2);

        System.out.println("--- 实例计数器 ---");
        new InitCounter();
        new InitCounter();
        System.out.println("total=" + InitCounter.created);

        System.out.println("done");
    }
}
