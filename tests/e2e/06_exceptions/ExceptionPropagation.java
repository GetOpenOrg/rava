import java.util.concurrent.CompletableFuture;

public class ExceptionPropagation {
    public static void main(String[] args) throws Exception {
        // Test 1: exceptionally recovery when exception occurs
        String r1 = CompletableFuture
            .<String>supplyAsync(() -> { throw new RuntimeException("fail"); })
            .exceptionally(e -> "recovered")
            .get();
        System.out.println(r1);

        // Test 2: exceptionally not called on success
        String r2 = CompletableFuture
            .completedFuture("ok")
            .exceptionally(e -> "should-not-appear")
            .get();
        System.out.println(r2);

        // Test 3: exception argument is non-null
        String r3 = CompletableFuture
            .<String>supplyAsync(() -> { throw new RuntimeException("test"); })
            .exceptionally(e -> e != null ? "non-null" : "null")
            .get();
        System.out.println(r3);

        // Test 4: thenApply chain fails -> exceptionally recovers
        String r4 = CompletableFuture
            .completedFuture("hello")
            .<String>thenApply(s -> { throw new RuntimeException("apply-fail"); })
            .exceptionally(e -> "apply-recovered")
            .get();
        System.out.println(r4);
    }
}
