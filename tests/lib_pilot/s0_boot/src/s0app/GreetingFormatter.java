package s0app;

/** 由 @Bean 方法创建的普通对象（无注解），验证 @Configuration 工厂方法注入 */
public class GreetingFormatter {
    private final String prefix;

    public GreetingFormatter(String prefix) {
        this.prefix = prefix;
    }

    public String format(String name) {
        return prefix + " " + name + "!";
    }
}
