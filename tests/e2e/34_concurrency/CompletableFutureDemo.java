import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;

public class CompletableFutureDemo {
    public static void main(String[] args) throws Exception {
        // completedFuture
        CompletableFuture<String> cf1 = CompletableFuture.completedFuture("hello");
        System.out.println(cf1.get());
        System.out.println(cf1.isDone());
        System.out.println(!cf1.isCancelled());

        // thenApply
        CompletableFuture<Integer> cf2 = CompletableFuture
            .completedFuture("world")
            .thenApply(String::length);
        System.out.println(cf2.get());

        // thenApply chain
        CompletableFuture<String> cf3 = CompletableFuture
            .completedFuture(5)
            .thenApply(n -> n * 2)
            .thenApply(n -> "result: " + n);
        System.out.println(cf3.get());

        // complete manually
        CompletableFuture<Integer> cf4 = new CompletableFuture<>();
        cf4.complete(42);
        System.out.println(cf4.get());
        System.out.println(cf4.isDone());

        // thenAccept (returns void CompletableFuture)
        CompletableFuture<String> cf5 = CompletableFuture.completedFuture("accept");
        cf5.thenAccept(s -> System.out.println("accepted: " + s)).get();

        // exceptionally
        CompletableFuture<Integer> cf6 = CompletableFuture
            .completedFuture(0)
            .thenApply(n -> {
                if (n == 0) throw new RuntimeException("zero!");
                return n;
            })
            .exceptionally(ex -> -1);
        System.out.println(cf6.get());

        System.out.println("done");
    }
}
