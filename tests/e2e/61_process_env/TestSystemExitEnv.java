import java.util.Map;

/** System.getenv（单键 / 全表）与 System.exit：exit 运行 shutdown hook 后以给定状态退出，其后代码不执行。 */
public class TestSystemExitEnv {
    public static void main(String[] args) {
        String path = System.getenv("PATH");
        System.out.println("PATH present: " + (path != null && !path.isEmpty()));
        System.out.println("missing var: " + System.getenv("JAVA_RTA_SURELY_MISSING_VAR_42"));
        Map<String, String> env = System.getenv();
        System.out.println("map has PATH: " + env.containsKey("PATH") + " consistent: " + env.get("PATH").equals(path));
        try {
            env.put("X", "Y");
        } catch (UnsupportedOperationException e) {
            System.out.println("env map unmodifiable");
        }
        Runtime.getRuntime().addShutdownHook(new Thread(() -> System.out.println("hook on exit")));
        System.out.println("before exit");
        System.exit(0);
        System.out.println("after exit must not print");
    }
}
