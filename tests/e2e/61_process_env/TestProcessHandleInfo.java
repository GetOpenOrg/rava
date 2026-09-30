import java.util.*;
import java.util.stream.*;

/** ProcessHandle 的进程信息与枚举：Info.info0 / getProcessPids0 / isAlive0 / parent0。 */
public class TestProcessHandleInfo {
    public static void main(String[] args) throws Exception {
        ProcessHandle self = ProcessHandle.current();
        System.out.println("self pid positive: " + (self.pid() > 0));
        System.out.println("self alive: " + self.isAlive());
        ProcessHandle.Info info = self.info();
        System.out.println("self start present: " + info.startInstant().isPresent());
        System.out.println("self start in past: "
            + info.startInstant().map(t -> !t.isAfter(java.time.Instant.now())).orElse(false));
        System.out.println("self cpu present: " + info.totalCpuDuration().isPresent());
        System.out.println("self user matches: "
            + info.user().map(u -> u.equals(System.getProperty("user.name"))).orElse(false));
        System.out.println("self command present: " + info.command().isPresent());
        System.out.println("self parent present: " + self.parent().isPresent());

        Process p = new ProcessBuilder("sleep", "5").start();
        ProcessHandle child = p.toHandle();
        List<Long> kids = self.children().map(ProcessHandle::pid).collect(Collectors.toList());
        System.out.println("child listed: " + kids.contains(child.pid()));
        System.out.println("child in descendants: "
            + self.descendants().anyMatch(h -> h.pid() == child.pid()));
        System.out.println("child parent is self: "
            + child.parent().map(h -> h.pid() == self.pid()).orElse(false));
        System.out.println("all processes contain self: "
            + ProcessHandle.allProcesses().anyMatch(h -> h.pid() == self.pid()));
        ProcessHandle.Info ci = child.info();
        System.out.println("child command: "
            + ci.command().map(c -> c.substring(c.lastIndexOf('/') + 1)).orElse("?"));
        System.out.println("child arguments: " + ci.arguments().map(Arrays::toString).orElse("?"));
        System.out.println("child commandLine ends: "
            + ci.commandLine().map(c -> c.endsWith("sleep 5")).orElse(false));
        System.out.println("child start present: " + ci.startInstant().isPresent());
        System.out.println("child equals by pid: " + ProcessHandle.of(child.pid()).map(child::equals).orElse(false));
        child.destroyForcibly();
        int code = p.waitFor();
        System.out.println("child exit: " + code);
        System.out.println("child alive after: " + child.isAlive());
    }
}
