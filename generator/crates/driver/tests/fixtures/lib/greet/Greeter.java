package greet;

/** lib crate 夹具：public 成员跨 crate 可见，包可见成员降级为 pub(crate) */
public class Greeter {
    public String greet(String name) {
        return prefix() + name;
    }

    String prefix() {
        return "hi ";
    }
}
