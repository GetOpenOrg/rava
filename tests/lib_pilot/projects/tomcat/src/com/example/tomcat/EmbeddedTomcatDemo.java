package com.example.tomcat;

import java.io.File;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.file.Files;

import org.apache.catalina.Context;
import org.apache.catalina.LifecycleException;
import org.apache.catalina.connector.Connector;
import org.apache.catalina.startup.Tomcat;

import jakarta.servlet.http.HttpServlet;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;

/**
 * 内嵌 Tomcat 编程式启动（对应文章"等效代码"一节）：
 * - 没有 server.xml、没有 web.xml、没有 Bootstrap 进程
 * - new Tomcat() 即创建 StandardServer→StandardService→StandardEngine→Host（均为 NEW 状态）
 * - tomcat.start() 触发 LifecycleBase.start()：NEW 状态自动先 init() 再 startInternal()
 * - Servlet 通过编程式动态注册（对应 TomcatStarter/ServletContextInitializer 的角色）
 *
 * 输出确定性说明：随机端口不打印，不打印任何耗时/标识符。
 */
public class EmbeddedTomcatDemo {

    public static void main(String[] args) throws Exception {
        // 关闭 Tomcat 的 JUL 日志（带时间戳/端口，破坏输出确定性）
        java.util.logging.Logger root = java.util.logging.Logger.getLogger("");
        root.setLevel(java.util.logging.Level.OFF);
        for (java.util.logging.Handler h : root.getHandlers()) {
            h.setLevel(java.util.logging.Level.OFF);
        }

        // 1. 创建内嵌工具类 Tomcat（不是 Bootstrap！）
        Tomcat tomcat = new Tomcat();

        // 2. 编程式添加 NIO Connector（端口 0 = 随机分配，验证用）
        File baseDir = Files.createTempDirectory("tomcat-embed").toFile();
        tomcat.setBaseDir(baseDir.getAbsolutePath());
        Connector connector = new Connector("org.apache.coyote.http11.Http11NioProtocol");
        connector.setPort(0);
        tomcat.getService().addConnector(connector);
        tomcat.setConnector(connector);

        // 3. 编程式创建 Context + 动态注册 Servlet（取代 web.xml）
        Context ctx = tomcat.addContext("", baseDir.getAbsolutePath());
        Tomcat.addServlet(ctx, "hello", new HttpServlet() {
            @Override
            protected void doGet(HttpServletRequest req, HttpServletResponse resp)
                    throws java.io.IOException {
                resp.setContentType("text/plain;charset=UTF-8");
                resp.getWriter().write("hello from embedded tomcat");
            }
        });
        ctx.addServletMappingDecoded("/hello", "hello");

        // 4. 启动：server.start() 是真正的点火按钮（自动先 init 再 start）
        tomcat.start();
        int port = connector.getLocalPort();
        System.out.println("内嵌 Tomcat 已编程式启动（无 server.xml / 无 web.xml）");

        // 5. 用 JDK HttpClient 实际请求验证
        HttpClient client = HttpClient.newHttpClient();
        HttpResponse<String> resp = client.send(
                HttpRequest.newBuilder(URI.create("http://localhost:" + port + "/hello")).GET().build(),
                HttpResponse.BodyHandlers.ofString());
        System.out.println("GET /hello -> " + resp.statusCode() + ": " + resp.body());

        // 6. 停止
        tomcat.stop();
        tomcat.destroy();
        System.out.println("内嵌 Tomcat 已停止");
    }
}
