import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

import com.sun.net.httpserver.HttpServer;

/**
 * java.net.http + jdk.httpserver：回环 HttpServer 多 context 的同步请求
 * （jmod 覆盖计划 B 档；临时端口不打印，服务端与客户端同进程，回环地址）。
 */
public class TestHttpLoopbackSync {

    static String base(HttpServer s) {
        return "http://" + s.getAddress().getHostString() + ":" + s.getAddress().getPort();
    }

    public static void main(String[] args) throws Exception {
        HttpServer server = HttpServer.create(
                new InetSocketAddress(InetAddress.getLoopbackAddress(), 0), 0);
        server.createContext("/hello", ex -> {
            byte[] body = "hello-http".getBytes("UTF-8");
            ex.getResponseHeaders().add("X-E2E", "v1");
            ex.sendResponseHeaders(200, body.length);
            ex.getResponseBody().write(body);
            ex.close();
        });
        server.createContext("/echo", ex -> {
            byte[] all = ex.getRequestBody().readAllBytes();
            ex.sendResponseHeaders(201, all.length);
            ex.getResponseBody().write(all);
            ex.close();
        });
        server.start();
        try {
            HttpClient cl = HttpClient.newHttpClient();

            HttpResponse<String> r1 = cl.send(
                    HttpRequest.newBuilder(URI.create(base(server) + "/hello")).GET().build(),
                    HttpResponse.BodyHandlers.ofString());
            System.out.println("code=" + r1.statusCode());
            System.out.println("body=" + r1.body());
            System.out.println("hdr=" + r1.headers().firstValue("X-E2E").orElse("none"));
            System.out.println("uri-path=" + r1.uri().getPath());

            HttpResponse<String> r2 = cl.send(
                    HttpRequest.newBuilder(URI.create(base(server) + "/echo"))
                            .POST(HttpRequest.BodyPublishers.ofString("ping-body")).build(),
                    HttpResponse.BodyHandlers.ofString());
            System.out.println("echo-code=" + r2.statusCode() + " echo-body=" + r2.body());

            HttpResponse<String> r3 = cl.send(
                    HttpRequest.newBuilder(URI.create(base(server) + "/missing")).GET().build(),
                    HttpResponse.BodyHandlers.ofString());
            System.out.println("not-found=" + r3.statusCode());
        } finally {
            server.stop(0);
        }
    }
}
