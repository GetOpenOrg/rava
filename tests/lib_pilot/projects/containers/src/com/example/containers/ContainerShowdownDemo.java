package com.example.containers;

import java.io.File;
import java.net.InetSocketAddress;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.file.Files;

import org.apache.catalina.Context;
import org.apache.catalina.connector.Connector;
import org.apache.catalina.startup.Tomcat;

import org.eclipse.jetty.server.Server;
import org.eclipse.jetty.server.ServerConnector;
import org.eclipse.jetty.servlet.ServletContextHandler;

import io.undertow.Undertow;

import io.netty.bootstrap.ServerBootstrap;
import io.netty.buffer.Unpooled;
import io.netty.channel.Channel;
import io.netty.channel.ChannelInitializer;
import io.netty.channel.nio.NioEventLoopGroup;
import io.netty.channel.socket.SocketChannel;
import io.netty.channel.socket.nio.NioServerSocketChannel;
import io.netty.handler.codec.http.DefaultFullHttpResponse;
import io.netty.handler.codec.http.FullHttpRequest;
import io.netty.handler.codec.http.HttpObjectAggregator;
import io.netty.handler.codec.http.HttpRequestDecoder;
import io.netty.handler.codec.http.HttpResponseEncoder;
import io.netty.handler.codec.http.HttpResponseStatus;
import io.netty.handler.codec.http.HttpVersion;
import io.netty.util.CharsetUtil;

import jakarta.servlet.http.HttpServlet;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;

/**
 * 四容器功能对照（对应文章§一优点对比 + §六压测结论）：
 * 同一个 /hello 端点分别用 Tomcat / Jetty / Undertow / Netty 内嵌启动，
 * JDK HttpClient 逐一验证响应，证明功能等价——性能差异（Netty ≈ Undertow > Jetty > Tomcat）
 * 需要文章的压测方法（ab -n 10000 -c 100）才能体现。
 *
 * 输出确定性说明：固定启动顺序、固定文案；随机端口不打印（Undertow 用固定端口）。
 */
public class ContainerShowdownDemo {

    private static final HttpClient HTTP = HttpClient.newHttpClient();

    public static void main(String[] args) throws Exception {
        // 关闭 Tomcat 的 JUL 日志（带时间戳/端口，破坏输出确定性）
        java.util.logging.Logger root = java.util.logging.Logger.getLogger("");
        root.setLevel(java.util.logging.Level.OFF);
        for (java.util.logging.Handler h : root.getHandlers()) {
            h.setLevel(java.util.logging.Level.OFF);
        }

        // ---------- 1. Tomcat（Servlet 容器，Spring Boot 默认） ----------
        File baseDir = Files.createTempDirectory("tomcat-showdown").toFile();
        Tomcat tomcat = new Tomcat();
        tomcat.setBaseDir(baseDir.getAbsolutePath());
        Connector connector = new Connector("org.apache.coyote.http11.Http11NioProtocol");
        connector.setPort(0);
        tomcat.getService().addConnector(connector);
        tomcat.setConnector(connector);
        Context ctx = tomcat.addContext("", baseDir.getAbsolutePath());
        Tomcat.addServlet(ctx, "hello", helloServlet("tomcat"));
        ctx.addServletMappingDecoded("/hello", "hello");
        tomcat.start();
        getAndPrint("Tomcat  ", "http://localhost:" + connector.getLocalPort() + "/hello");
        tomcat.stop();
        tomcat.destroy();

        // ---------- 2. Jetty（轻量、可定制，Servlet 容器） ----------
        Server jetty = new Server();
        ServerConnector jettyConnector = new ServerConnector(jetty);
        jettyConnector.setPort(0);
        jetty.addConnector(jettyConnector);
        ServletContextHandler jettyHandler = new ServletContextHandler();
        jettyHandler.setContextPath("/");
        jettyHandler.addServlet(new org.eclipse.jetty.servlet.ServletHolder(helloServlet("jetty")), "/hello");
        jetty.setHandler(jettyHandler);
        jetty.start();
        getAndPrint("Jetty   ", "http://localhost:" + jettyConnector.getLocalPort() + "/hello");
        jetty.stop();

        // ---------- 3. Undertow（非阻塞 IO，非 Servlet 编程模型） ----------
        Undertow undertow = Undertow.builder()
                .addHttpListener(18083, "localhost")
                .setHandler(exchange -> {
                    if ("/hello".equals(exchange.getRequestPath())) {
                        exchange.getResponseSender().send("hello from undertow");
                    } else {
                        exchange.setStatusCode(404);
                    }
                })
                .build();
        undertow.start();
        getAndPrint("Undertow", "http://localhost:18083/hello");
        undertow.stop();

        // ---------- 4. Netty（异步事件驱动，WebFlux 底层） ----------
        NioEventLoopGroup boss = new NioEventLoopGroup(1);
        NioEventLoopGroup worker = new NioEventLoopGroup(1);
        Channel nettyChannel = new ServerBootstrap()
                .group(boss, worker)
                .channel(NioServerSocketChannel.class)
                .childHandler(new ChannelInitializer<SocketChannel>() {
                    @Override
                    protected void initChannel(SocketChannel ch) {
                        ch.pipeline().addLast(
                                new HttpRequestDecoder(),
                                new HttpObjectAggregator(8192),
                                new HttpResponseEncoder(),
                                new io.netty.channel.SimpleChannelInboundHandler<FullHttpRequest>() {
                                    @Override
                                    protected void channelRead0(io.netty.channel.ChannelHandlerContext ctx,
                                                                 FullHttpRequest req) {
                                        boolean keepAlive =
                                                io.netty.handler.codec.http.HttpUtil.isKeepAlive(req);
                                        DefaultFullHttpResponse resp = new DefaultFullHttpResponse(
                                                HttpVersion.HTTP_1_1, HttpResponseStatus.OK,
                                                Unpooled.copiedBuffer("hello from netty", CharsetUtil.UTF_8));
                                        resp.headers().set("Content-Length", resp.content().readableBytes());
                                        io.netty.handler.codec.http.HttpUtil.setKeepAlive(resp, keepAlive);
                                        ctx.writeAndFlush(resp)
                                                .addListener(io.netty.channel.ChannelFutureListener.CLOSE);
                                    }
                                });
                    }
                })
                .bind(0).sync().channel();
        int nettyPort = ((InetSocketAddress) nettyChannel.localAddress()).getPort();
        getAndPrint("Netty   ", "http://localhost:" + nettyPort + "/hello");
        nettyChannel.close().sync();
        boss.shutdownGracefully();
        worker.shutdownGracefully();

        System.out.println("结论: 四种容器功能等价；性能排序（文章压测）Netty ≈ Undertow > Jetty > Tomcat");
        System.out.println("选型: 普通业务选 Tomcat，追求轻量/性能选 Undertow，");
        System.out.println("      响应式/WebSocket/网关选 Netty，轻量定制选 Jetty");
        System.exit(0); // 确保容器线程不阻塞 JVM 退出
    }

    private static HttpServlet helloServlet(String who) {
        return new HttpServlet() {
            @Override
            protected void doGet(HttpServletRequest req, HttpServletResponse resp)
                    throws java.io.IOException {
                resp.setContentType("text/plain;charset=UTF-8");
                resp.getWriter().write("hello from " + who);
            }
        };
    }

    private static void getAndPrint(String label, String url) throws Exception {
        HttpResponse<String> resp = HTTP.send(
                HttpRequest.newBuilder(URI.create(url)).GET().build(),
                HttpResponse.BodyHandlers.ofString());
        System.out.println(label + " -> " + resp.statusCode() + ": " + resp.body());
    }
}
