interface Named {
    String name();
}

interface Described {
    String describe();
}

interface Greeter {
    default String hello() {
        return "hello from Greeter";
    }

    default String version() {
        return "greeter-v1";
    }
}

interface Speaker {
    default String hello() {
        return "hello from Speaker";
    }

    default String version() {
        return "speaker-v1";
    }
}

// 两个接口有同名默认方法 → 必须显式解决，并选择其中一个 version
class Impl implements Greeter, Speaker {
    @Override
    public String hello() {
        return Greeter.super.hello() + " + " + Speaker.super.hello();
    }

    @Override
    public String version() {
        return Speaker.super.version();
    }
}

// 只解决其中一个：另一个冲突方法也必须显式覆盖或此处也要解决
class OnlyGreeter implements Greeter, Speaker {
    @Override
    public String hello() {
        return Speaker.super.hello();
    }

    @Override
    public String version() {
        return Greeter.super.version();
    }
}

// 菱形：D -> B,C -> A
interface A0 {
    default String tag() {
        return "A";
    }
}

interface B0 extends A0 {
    @Override
    default String tag() {
        return "B";
    }
}

interface C0 extends A0 {
    @Override
    default String tag() {
        return "C";
    }
}

class Diamond implements B0, C0 {
    public String tag() {
        return B0.super.tag() + "/via-" + C0.super.tag();
    }
}

// 抽象类提供默认实现的同名方法：类优先于接口
abstract class AbstractHost {
    public String hello() {
        return "abstract-hello";
    }
}

class ClassWins extends AbstractHost implements Greeter {
    // 不重写 hello() → 走 AbstractHost 的实现
}

public class TestInterfaceConflict {

    public static void main(String[] args) {
        Impl impl = new Impl();
        System.out.println(impl.hello());
        System.out.println(impl.version());

        OnlyGreeter og = new OnlyGreeter();
        System.out.println(og.hello());
        System.out.println(og.version());

        Diamond d = new Diamond();
        System.out.println(d.tag());

        System.out.println(new ClassWins().hello());

        // 通过接口引用调用：仍解析到同一个重写方法
        Greeter g = impl;
        Speaker s = impl;
        System.out.println("via Greeter=" + g.hello());
        System.out.println("via Speaker=" + s.hello());
        System.out.println("via Speaker version=" + s.version());

        // 接口继承后叠加默认方法
        Named n = () -> "lambda-named";
        Described desc = () -> "lambda-described";
        System.out.println(n.name() + " / " + desc.describe());

        System.out.println("done");
    }
}
