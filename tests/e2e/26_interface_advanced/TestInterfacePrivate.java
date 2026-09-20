interface Pipeline {
    String execute(String input);

    // 接口私有方法（Java 9+）：只能被本接口默认方法复用，子类不可覆写
    private String trimInput(String s) {
        return s.trim();
    }

    private String upper(String s) {
        return s.toUpperCase();
    }

    default String normalize(String s) {
        return trimInput(s);
    }

    default String shout(String s) {
        return upper(trimInput(s)) + "!";
    }

    // 私有静态方法
    static String version() {
        return "pipeline-1.0";
    }

    private static String tag(String label) {
        return "[" + label + "]";
    }

    default String labelled(String s) {
        return tag(normalize(s));
    }
}

class SimplePipeline implements Pipeline {
    @Override
    public String execute(String input) {
        return normalize(input) + " executed";
    }
}

class ShoutPipeline implements Pipeline {
    @Override
    public String execute(String input) {
        return shout(input);
    }
}

public class TestInterfacePrivate {

    public static void main(String[] args) {
        Pipeline simple = new SimplePipeline();
        Pipeline shouty = new ShoutPipeline();

        System.out.println(simple.execute("  hello  "));
        System.out.println(shouty.execute("  hey  "));

        System.out.println("normalize=" + simple.normalize("  spaced  "));
        System.out.println("shout=" + simple.shout("  quiet  "));
        System.out.println("labelled=" + simple.labelled(" a "));

        // 接口静态方法 + 私有静态方法组合
        System.out.println("version=" + Pipeline.version());

        // 多个实现共存，依赖同一个私有辅助逻辑
        Pipeline[] pipelines = {simple, shouty};
        for (Pipeline p : pipelines) {
            System.out.println("run=" + p.execute(" x "));
        }

        // 私有方法不可被实现类同名的方法覆盖
        class OverrideAttempt implements Pipeline {
            @Override
            public String execute(String input) {
                return "custom:" + input;
            }

            String trimInput(String s) {
                return "not-used";
            }
        }
        Pipeline custom = new OverrideAttempt();
        System.out.println(custom.execute(" y "));
        System.out.println(custom.normalize(" z "));

        System.out.println("done");
    }
}
