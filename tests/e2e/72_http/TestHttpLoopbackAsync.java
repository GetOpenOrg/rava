import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.concurrent.CompletableFuture;

import com.sun.net.httpserver.HttpServer;

/**
 * java.net.http 异步路径：sendAsync + CompletableFuture 组合
 * （jmod 覆盖计划 B 档；NIO selector 线程与 Executor，输出只在 join 后打印，
 * 顺序与调度解耦）。
 */
public class TestHttpLoopbackAsync {

    public static void main(String[] args) throws Exception {
        HttpServer server = HttpServer.create(
                new InetSocketAddress(InetAddress.getLoopbackAddress(), 0), 0);
        server.createContext("/a", ex -> respond(ex, "alpha"));
        server.createContext("/b", ex -> respond(ex, "beta"));
        server.start();
        try {
            String base = "http://" + server.getAddress().getHostString() + ":"
                    + server.getAddress().getPort();
            HttpClient cl = HttpClient.newHttpClient();

            CompletableFuture<HttpResponse<String>> fa = cl.sendAsync(
                    HttpRequest.newBuilder(URI.create(base + "/a")).GET().build(),
                    HttpResponse.BodyHandlers.ofString());
            CompletableFuture<HttpResponse<String>> fb = cl.sendAsync(
                    HttpRequest.newBuilder(URI.create(base + "/b")).GET().build(),
                    HttpResponse.BodyHandlers.ofString());

            fa.thenCombine(fb, (ra, rb) -> ra.statusCode() + "&" + ra.body() + "|"
                    + rb.statusCode() + "&" + rb.body())
                    .thenAccept(System.out::println)
                    .join();

            // 依赖链：两段请求串行
            cl.sendAsync(HttpRequest.newBuilder(URI.create(base + "/a")).GET().build(),
                    HttpResponse.BodyHandlers.ofString())
                    .thenCompose(r -> {
                        System.out.println("chain-first=" + r.body());
                        return cl.sendAsync(HttpRequest.newBuilder(URI.create(base + "/b")).GET().build(),
                                HttpResponse.BodyHandlers.ofString());
                    })
                    .thenAccept(r -> System.out.println("chain-second=" + r.body()))
                    .join();

            // 异常路径：对不存在 context 的异步请求 → 404 Future
            cl.sendAsync(HttpRequest.newBuilder(URI.create(base + "/none")).GET().build(),
                    HttpResponse.BodyHandlers.ofString())
                    .thenApply(HttpResponse::statusCode)
                    .thenAccept(sc -> System.out.println("async-404=" + sc))
                    .join();
        } finally {
            server.stop(0);
        }
    }

    private static void respond(com.sun.net.httpserver.HttpExchange ex, String body)
            throws java.io.IOException {
        byte[] b = body.getBytes("UTF-8");
        ex.sendResponseHeaders(200, b.length);
        ex.getResponseBody().write(b);
        ex.close();
    }
}
