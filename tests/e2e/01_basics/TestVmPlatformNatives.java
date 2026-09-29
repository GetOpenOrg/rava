import java.lang.reflect.Array;
import java.time.Instant;
import java.util.Arrays;

public class TestVmPlatformNatives {
    public static void main(String[] args) {
        // VM.getNanoTimeAdjustment（Instant.now）：与 currentTimeMillis 相差不超过 10 秒
        long nowMs = System.currentTimeMillis();
        Instant now = Instant.now();
        System.out.println("instant-close=" + (Math.abs(now.toEpochMilli() - nowMs) < 10_000));
        System.out.println("instant-nanos-ok=" + (now.getNano() >= 0 && now.getNano() < 1_000_000_000));

        // Runtime 内存：maxMemory 为正，totalMemory - freeMemory 非负
        Runtime rt = Runtime.getRuntime();
        rt.gc();
        System.out.println("max-positive=" + (rt.maxMemory() > 0));
        System.out.println("used-nonneg=" + (rt.totalMemory() - rt.freeMemory() >= 0));

        // System.mapLibraryName
        System.out.println(System.mapLibraryName("zip"));

        // ProcessHandle：当前 pid 为正且存活，父进程存在
        ProcessHandle self = ProcessHandle.current();
        System.out.println("pid-positive=" + (self.pid() > 0));
        System.out.println("alive=" + self.isAlive());
        System.out.println("has-parent=" + self.parent().isPresent());

        // Array.multiNewArray（Array.newInstance 多维）
        int[][] grid = (int[][]) Array.newInstance(int.class, 2, 3);
        grid[1][2] = 7;
        System.out.println(Arrays.deepToString(grid) + " " + grid.getClass().getName());
        String[][][] cube = (String[][][]) Array.newInstance(String.class, 1, 2, 2);
        cube[0][1][0] = "x";
        System.out.println(Arrays.deepToString(cube) + " " + cube.getClass().getSimpleName());
        try {
            Array.newInstance(int.class, 2, -1);
        } catch (NegativeArraySizeException e) {
            System.out.println("NASE");
        }
    }
}
