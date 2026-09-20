import java.util.concurrent.*;

public class TestCompletableFuture {
    public static void main(String[] args) throws Exception {
        CompletableFuture<Integer> f = CompletableFuture.supplyAsync(() -> 21).thenApply(x -> x * 2);
        System.out.println("thenApply=" + f.get());

        CompletableFuture<String> g = CompletableFuture.supplyAsync(() -> "a")
                .thenCombine(CompletableFuture.supplyAsync(() -> "b"), (a, b) -> a + b);
        System.out.println("combine=" + g.get());

        CompletableFuture<Integer> h = CompletableFuture.supplyAsync(() -> 7);
        System.out.println("async=" + h.get());

        CompletableFuture<Integer> all = CompletableFuture.allOf(f, g).thenApply(v -> 99);
        System.out.println("allOf=" + all.get());

        CompletableFuture<Integer> exceptionally = CompletableFuture
                .<Integer>supplyAsync(() -> { throw new RuntimeException("x"); })
                .exceptionally(ex -> 0);
        System.out.println("exceptionally=" + exceptionally.get());
    }
}
