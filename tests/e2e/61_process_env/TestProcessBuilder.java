import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.List;

// 进程派生：ProcessImpl.forkAndExec + ProcessHandleImpl.waitForProcessExit0
public class TestProcessBuilder {
    static String readAll(java.io.InputStream in) throws IOException {
        StringBuilder sb = new StringBuilder();
        try (BufferedReader r = new BufferedReader(new InputStreamReader(in, StandardCharsets.UTF_8))) {
            String line;
            while ((line = r.readLine()) != null) {
                sb.append(line).append('|');
            }
        }
        return sb.toString();
    }

    public static void main(String[] args) throws Exception {
        // 1. 标准输出 / 标准错误分离 + 退出码
        Process p = new ProcessBuilder("sh", "-c", "echo out1; echo err1 1>&2; echo out2; exit 3").start();
        System.out.println("stdout=" + readAll(p.getInputStream()));
        System.out.println("stderr=" + readAll(p.getErrorStream()));
        System.out.println("exit=" + p.waitFor());
        System.out.println("exitValue=" + p.exitValue() + " alive=" + p.isAlive());

        // 2. 合并错误流：getErrorStream 为空流
        ProcessBuilder pb = new ProcessBuilder("sh", "-c", "echo a; echo b 1>&2; echo c");
        pb.redirectErrorStream(true);
        Process p2 = pb.start();
        System.out.println("merged=" + readAll(p2.getInputStream()));
        System.out.println("errAfterMerge=" + p2.getErrorStream().read());
        System.out.println("exit2=" + p2.waitFor());

        // 3. 环境变量（修改后传入完整环境块）
        ProcessBuilder pb3 = new ProcessBuilder("sh", "-c", "echo \"v=$RAVA_TEST_VAR\"");
        pb3.environment().put("RAVA_TEST_VAR", "hello-env");
        Process p3 = pb3.start();
        System.out.println(readAll(p3.getInputStream()) + " exit3=" + p3.waitFor());

        // 4. 工作目录
        ProcessBuilder pb4 = new ProcessBuilder("pwd");
        pb4.directory(new java.io.File("/"));
        Process p4 = pb4.start();
        System.out.println("pwd=" + readAll(p4.getInputStream()) + " exit4=" + p4.waitFor());

        // 5. 标准输入管道
        Process p5 = new ProcessBuilder("sh", "-c", "read x; echo \"got:$x\"; read y; echo \"got:$y\"").start();
        try (OutputStream os = p5.getOutputStream()) {
            os.write("line-one\nline-two\n".getBytes(StandardCharsets.UTF_8));
        }
        System.out.println(readAll(p5.getInputStream()) + " exit5=" + p5.waitFor());

        // 6. 被信号终止的退出码
        Process p6 = new ProcessBuilder("sh", "-c", "kill -9 $$").start();
        System.out.println("killed=" + p6.waitFor());

        // 7. 找不到程序
        try {
            new ProcessBuilder("rava-no-such-program-xyz").start();
            System.out.println("unexpected success");
        } catch (IOException e) {
            System.out.println("IOException: " + e.getMessage());
        }

        // 8. Runtime.exec + 参数含空格
        Process p8 = Runtime.getRuntime().exec(new String[]{"printf", "%s;%s", "a b", "c"});
        System.out.println("printf=" + readAll(p8.getInputStream()) + " exit8=" + p8.waitFor());

        // 9. 继承标准输出（子进程直接写到本进程 stdout）
        System.out.flush();
        Process p9 = new ProcessBuilder(List.of("echo", "inherited-line")).inheritIO().start();
        System.out.println("exit9=" + p9.waitFor());
    }
}
