package com.example.kotlinlib;

import java.io.IOException;

import okhttp3.MediaType;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.RequestBody;
import okhttp3.Response;
import okhttp3.mockwebserver.MockResponse;
import okhttp3.mockwebserver.MockWebServer;
import okhttp3.mockwebserver.RecordedRequest;

/**
 * Java 调用 Kotlin 库：OkHttp（4.0 起整体用 Kotlin 重写）。
 * "大量 Java 项目天天在调它而不自知"——Java 调用体验与 Java 库无异。
 *
 * 用 MockWebServer（同为 Kotlin）提供本地回环响应，不访问外网，
 * 保证输出确定性；同时用 takeRequest() 展示服务端视角收到的请求。
 *
 * 输出确定性说明：URL 由 MockWebServer 随机端口生成，但不打印 URL 本身。
 */
public class OkHttpDemo {

    public static void main(String[] args) throws Exception {
        MockWebServer server = new MockWebServer();
        server.enqueue(new MockResponse()
                .setBody("hello from mockwebserver")
                .setHeader("Content-Type", "text/plain"));
        server.enqueue(new MockResponse()
                .setBody("{\"ok\":true}")
                .setHeader("Content-Type", "application/json"));
        server.start();

        OkHttpClient client = new OkHttpClient();

        // ---------- 1. GET ----------
        Request get = new Request.Builder()
                .url(server.url("/hello").toString())
                .header("X-Demo", "java-calls-kotlin")
                .build();
        try (Response resp = client.newCall(get).execute()) {
            System.out.println("[GET] " + resp.code() + " "
                    + resp.header("Content-Type") + " -> " + resp.body().string());
        }

        // ---------- 2. POST ----------
        Request post = new Request.Builder()
                .url(server.url("/submit").toString())
                .post(RequestBody.create("payload-数据", MediaType.parse("text/plain")))
                .build();
        try (Response resp = client.newCall(post).execute()) {
            System.out.println("[POST] " + resp.code() + " -> " + resp.body().string());
        }

        // ---------- 3. 服务端视角：MockWebServer 收到的请求 ----------
        RecordedRequest recorded1 = server.takeRequest();
        System.out.println("[Server收到] " + recorded1.getMethod() + " "
                + recorded1.getPath() + " header(X-Demo)=" + recorded1.getHeader("X-Demo"));
        RecordedRequest recorded2 = server.takeRequest();
        System.out.println("[Server收到] " + recorded2.getMethod() + " "
                + recorded2.getPath() + " body=" + recorded2.getBody().readUtf8());

        server.shutdown();
        System.out.println("[结论] OkHttp 4.x 是纯 Kotlin 库，以上全部是 Java 代码在调用");
    }
}
